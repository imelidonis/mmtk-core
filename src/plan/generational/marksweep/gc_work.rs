use super::global::GenMarkSweep;
use crate::plan::generational::gc_work::GenNurseryProcessEdges;
use crate::vm::VMBinding;

use crate::policy::gc_work::DEFAULT_TRACE;
use crate::scheduler::gc_work::{PlanProcessEdges, UnsupportedProcessEdges};

pub struct GenMarkSweepNurseryGCWorkContext<VM: VMBinding>(std::marker::PhantomData<VM>);
impl<VM: VMBinding> crate::scheduler::GCWorkContext for GenMarkSweepNurseryGCWorkContext<VM> {
    type VM = VM;
    type PlanType = GenMarkSweep<VM>;
    type ProcessEdgesWorkType = GenNurseryProcessEdges<VM, Self::PlanType>;
    type TPProcessEdges = UnsupportedProcessEdges<VM>;
}

pub struct GenMarkSweepGCWorkContext<VM: VMBinding>(std::marker::PhantomData<VM>);
impl<VM: VMBinding> crate::scheduler::GCWorkContext for GenMarkSweepGCWorkContext<VM> {
    type VM = VM;
    type PlanType = GenMarkSweep<VM>;
    type ProcessEdgesWorkType = PlanProcessEdges<Self::VM, GenMarkSweep<VM>, DEFAULT_TRACE>;
    type TPProcessEdges = UnsupportedProcessEdges<VM>;
}

