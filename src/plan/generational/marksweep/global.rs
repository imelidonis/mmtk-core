use crate::plan::global::{CommonPlan, CreateSpecificPlanArgs};
use crate::plan::{CreateGeneralPlanArgs, GenerationalPlan};
use crate::policy::marksweepspace::native_ms::MarkSweepSpace;
use crate::policy::space::Space;
use crate::util::copy::{CopyConfig, CopySelector, CopySemantics};
use crate::util::heap::VMRequest;
use crate::Plan;
use crate::{plan::PlanConstraints, vm::VMBinding};
use crate::plan::generational::global::{CommonGenPlan, GenerationalPlanExt};

use mmtk_macros::{HasSpaces, PlanTraceObject};

use super::gc_work::GenMarkSweepGCWorkContext;
use super::gc_work::GenMarkSweepNurseryGCWorkContext;
use super::mutator::ALLOCATOR_MAPPING;

#[derive(HasSpaces, PlanTraceObject)]
pub struct GenMarkSweep<VM: VMBinding> {
    #[space]
    ms: MarkSweepSpace<VM>,
    #[parent]
    pub gen: CommonGenPlan<VM>,
}

/// The plan constraints for the generational mark sweep plan.
pub const GENMS_CONSTRAINTS: PlanConstraints = crate::plan::generational::GEN_CONSTRAINTS;

impl<VM: VMBinding> Plan for GenMarkSweep<VM> {
    fn constraints(&self) -> &'static PlanConstraints {
        &GENMS_CONSTRAINTS
    }

    fn create_copy_config(&'static self) -> CopyConfig<Self::VM> {
        use enum_map::enum_map;
        CopyConfig {
            copy_mapping: enum_map! {
                CopySemantics::PromoteToMature => CopySelector::MarkSweepSpace(0),
                _ => CopySelector::Unused,
            },
            space_mapping: vec![
                (CopySelector::MarkSweepSpace(0), self.ms_space())
            ],
            constraints: &GENMS_CONSTRAINTS,
        }
    }

    fn base(&self) -> &crate::plan::global::BasePlan<Self::VM> {
        &self.gen.common.base
    }

    fn base_mut(&mut self) -> &mut crate::plan::global::BasePlan<Self::VM> {
        &mut self.gen.common.base
    }

    fn schedule_collection(&'static self, scheduler: &crate::scheduler::GCWorkScheduler<Self::VM>) {
        let is_full_heap = self.requires_full_heap_collection();
        if is_full_heap {
            scheduler.schedule_common_work::<GenMarkSweepGCWorkContext<VM>>(self);
        } else {
            scheduler.schedule_common_work::<GenMarkSweepNurseryGCWorkContext<VM>>(self);
        }
    }

    fn get_allocator_mapping(&self) -> &'static enum_map::EnumMap<crate::AllocationSemantics, crate::util::alloc::AllocatorSelector> {
        &ALLOCATOR_MAPPING
    }

    fn prepare(&mut self, tls: crate::util::VMWorkerThread) {
        let full_heap = !self.is_current_gc_nursery();
        self.gen.prepare(tls);
        if full_heap {
            self.ms_space_mut().prepare()
        }
    }

    fn release(&mut self, tls: crate::util::VMWorkerThread) {
        let full_heap = !self.is_current_gc_nursery();
        self.gen.release(tls);
        if full_heap {
            self.ms_space_mut().release();
        }
    }

    fn end_of_gc(&mut self, _tls: crate::util::VMWorkerThread) {
        self.gen
            .set_next_gc_full_heap(CommonGenPlan::should_next_gc_be_full_heap(self))
    }

    fn collection_required(&self, space_full: bool, space: Option<crate::util::heap::SpaceStats<Self::VM>>) -> bool {
        self.gen.collection_required(self, space_full, space)
    }

    fn get_collection_reserved_pages(&self) -> usize {
        self.gen.get_collection_reserved_pages() + self.ms_space().reserved_pages()
    }

    fn get_used_pages(&self) -> usize {
        self.gen.get_used_pages() + self.ms_space().reserved_pages()
    }

    fn common(&self) -> &CommonPlan<VM> {
        &self.gen.common
    }

    fn generational(&self) -> Option<&dyn GenerationalPlan<VM = Self::VM>> {
        Some(self)
    }
}

impl<VM: VMBinding> GenerationalPlan for GenMarkSweep<VM> {
    fn is_current_gc_nursery(&self) -> bool {
       self.gen.is_current_gc_nursery()
    }

    fn is_object_in_nursery(&self, object: crate::util::ObjectReference) -> bool {
        self.gen.nursery.in_space(object)
    }

    fn is_address_in_nursery(&self, addr: crate::util::Address) -> bool {
        self.gen.nursery.address_in_space(addr)
    }

    fn get_mature_physical_pages_available(&self) -> usize {
        self.ms_space().available_physical_pages()
    }

    fn get_mature_reserved_pages(&self) -> usize {
        self.ms_space().reserved_pages()
    }

    fn last_collection_full_heap(&self) -> bool {
        self.gen.last_collection_full_heap()
    }

    fn force_full_heap_collection(&self) {
        self.gen.force_full_heap_collection()
    }
}

impl<VM: VMBinding> GenerationalPlanExt<VM> for GenMarkSweep<VM> {
    fn trace_object_nursery<Q: crate::ObjectQueue>(
        &self,
        queue: &mut Q,
        object: crate::util::ObjectReference,
        worker: &mut crate::scheduler::GCWorker<VM>,
    ) -> crate::util::ObjectReference {
        self.gen.trace_object_nursery(queue, object, worker)
    }
}

impl<VM: VMBinding> GenMarkSweep<VM> {
    pub fn new(args: CreateGeneralPlanArgs<VM>) -> Self {
        let mut plan_args = CreateSpecificPlanArgs {
            global_args: args,
            constraints: &GENMS_CONSTRAINTS,
            global_side_metadata_specs:
                crate::plan::generational::new_generational_global_metadata_specs::<VM>(),
        };

        let res = GenMarkSweep {
            ms: MarkSweepSpace::new(plan_args.get_space_args(
                "ms",
                true,
                VMRequest::discontiguous()
            )),
            gen: CommonGenPlan::new(plan_args),
        };

        res.verify_side_metadata_sanity();

        res
    }

    fn requires_full_heap_collection(&self) -> bool {
        self.gen.requires_full_heap_collection(self)
    }

    pub fn ms_space(&self) -> &MarkSweepSpace<VM> {
        &self.ms
    }

    pub fn ms_space_mut(&mut self) -> &mut MarkSweepSpace<VM> {
        &mut self.ms
    }
}

