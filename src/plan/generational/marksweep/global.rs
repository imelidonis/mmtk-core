use crate::plan::global::CreateSpecificPlanArgs;
use crate::plan::CreateGeneralPlanArgs;
use crate::policy::marksweepspace::native_ms::MarkSweepSpace;
use crate::util::copy::CopySemantics;
use crate::util::heap::VMRequest;
use crate::{plan::PlanConstraints, vm::VMBinding};
use crate::plan::generational::global::CommonGenPlan;

use mmtk_macros::{HasSpaces, PlanTraceObject};

#[derive(HasSpaces, PlanTraceObject)]
pub struct GenMarkSweep<VM: VMBinding> {
    #[parent]
    pub gen: CommonGenPlan<VM>,
    // TODO: not sure...
    #[space]
    #[copy_semantics(CopySemantics::Mature)]
    ms: MarkSweepSpace<VM>,
}

/// The plan constraints for the generational mark sweep plan.
pub const GENMS_CONSTRAINTS: PlanConstraints = crate::plan::generational::GEN_CONSTRAINTS;

impl<VM: VMBinding> GenMarkSweep<VM> {
    pub fn new(args: CreateGeneralPlanArgs<VM>) -> Self {
        let mut plan_args = CreateSpecificPlanArgs {
            global_args: args,
            constraints: &GENMS_CONSTRAINTS,
            global_side_metadata_specs:
                crate::plan::generational::new_generational_global_metadata_specs::<VM>(),
        };

        let res = GenMarkSweep {
            gen: CommonGenPlan::new(plan_args),
            ms: MarkSweepSpace::new(plan_args.get_space_args(
                "ms",
                true,
                VMRequest::discontiguous()
            )),
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

