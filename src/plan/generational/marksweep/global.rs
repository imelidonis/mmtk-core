use crate::plan::global::CreateSpecificPlanArgs;
use crate::plan::{CreateGeneralPlanArgs, GenerationalPlan};
use crate::policy::marksweepspace::native_ms::MarkSweepSpace;
use crate::policy::space::Space;
use crate::util::copy::CopySemantics;
use crate::util::heap::VMRequest;
use crate::Plan;
use crate::{plan::PlanConstraints, vm::VMBinding};
use crate::plan::generational::global::{CommonGenPlan, GenerationalPlanExt};

use mmtk_macros::{HasSpaces, PlanTraceObject};

#[derive(HasSpaces, PlanTraceObject)]
pub struct GenMarkSweep<VM: VMBinding> {
    // TODO: not sure...
    #[space]
    #[copy_semantics(CopySemantics::Mature)]
    ms: MarkSweepSpace<VM>,
    #[parent]
    pub gen: CommonGenPlan<VM>,
}

/// The plan constraints for the generational mark sweep plan.
pub const GENMS_CONSTRAINTS: PlanConstraints = crate::plan::generational::GEN_CONSTRAINTS;

impl<VM: VMBinding> Plan  for GenMarkSweep<VM> {
    fn constraints(&self) -> &'static PlanConstraints {
        todo!()
    }

    fn base(&self) -> &crate::plan::global::BasePlan<Self::VM> {
        todo!()
    }

    fn base_mut(&mut self) -> &mut crate::plan::global::BasePlan<Self::VM> {
        todo!()
    }

    fn schedule_collection(&'static self, _scheduler: &crate::scheduler::GCWorkScheduler<Self::VM>) {
        todo!()
    }

    fn get_allocator_mapping(&self) -> &'static enum_map::EnumMap<crate::AllocationSemantics, crate::util::alloc::AllocatorSelector> {
        todo!()
    }

    fn prepare(&mut self, tls: crate::util::VMWorkerThread) {
        todo!()
    }

    fn release(&mut self, tls: crate::util::VMWorkerThread) {
        todo!()
    }

    fn collection_required(&self, space_full: bool, space: Option<crate::util::heap::SpaceStats<Self::VM>>) -> bool {
        todo!()
    }

    fn get_used_pages(&self) -> usize {
        todo!()
    }
}

impl<VM: VMBinding> GenerationalPlan for GenMarkSweep<VM> {
    fn is_current_gc_nursery(&self) -> bool {
        todo!()
    }

    fn is_object_in_nursery(&self, object: crate::util::ObjectReference) -> bool {
        todo!()
    }

    fn is_address_in_nursery(&self, addr: crate::util::Address) -> bool {
        todo!()
    }

    fn get_mature_physical_pages_available(&self) -> usize {
        todo!()
    }

    fn get_mature_reserved_pages(&self) -> usize {
        todo!()
    }

    fn last_collection_full_heap(&self) -> bool {
        todo!()
    }

    fn force_full_heap_collection(&self) {
        todo!()
    }
}

impl<VM: VMBinding> GenerationalPlanExt<VM> for GenMarkSweep<VM> {
    fn trace_object_nursery<Q: crate::ObjectQueue>(
        &self,
        queue: &mut Q,
        object: crate::util::ObjectReference,
        worker: &mut crate::scheduler::GCWorker<VM>,
    ) -> crate::util::ObjectReference {
        todo!()
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
}

