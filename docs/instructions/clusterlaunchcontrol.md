### Description

The clusterlaunchcontrol.try_cancel instruction requests atomically cancelling the launch of
a cluster that has not started running yet. It asynchronously writes an opaque response to shared
memory indicating whether the operation succeeded or failed. The completion of the asynchronous
operation is tracked using the mbarrier completion mechanism at .cluster scope.
On success, the opaque response contains the ctaid of the first CTA of the canceled cluster; no
other successful response from other clusterlaunchcontrol.try_cancel operations from the same
grid will contain that id.
The mandatory .async qualifier indicates that the instruction will initiate the cancellation
operation asynchronously and control will return to the executing thread before the requested
operation is complete.
The .space qualifier is specified, both operands addr and mbar must be in the
.shared::cta state space. Otherwise, generic addressing will be assumed for both. The result
is undefined if any of address operands do not fall within the address window of .shared::cta.
The qualifier .completion_mechanism specifies that upon completion of the asynchronous operation,
complete-tx
operation, with completeCount argument equal to amount of data stored in bytes, will be performed
on the mbarrier object specified by the operand mbar.
The executing thread can then use mbarrier instructions to wait for completion
of the asynchronous operation. No other synchronization mechanisms described in Memory Consistency Model can be used to guarantee the completion of the asynchronous copy operations.
The .multicast::cluster::all qualifier indicates that the response is asynchronously written using
weak async-proxy writes to the corresponding local shared memory addr of each CTA in the requesting
cluster. The completion of the writes to addr of a particular CTA is signaled via a complete-tx operation
to the mbarrier object on the shared memory of that CTA.
The behavior of instruction with .multicast::cluster::all qualifier is undefined if any CTA in the
cluster is exited.
Operand addr specifies the naturally aligned address of the 16-byte wide shared memory location where
the request’s response is written.
The response of clusterlaunchcontrol.try_cancel instruction will be 16-byte opaque value and will be
it available at location specified by operand addr. After loading this response into 16-byte register,
instruction clusterlaunchcontrol.query_cancel can be used to check if request was successful and to
retrieve ctaid of the first CTA of the canceled cluster.
If the executing CTA has already observed the completion of a clusterlaunchcontrol.try_cancel instruction
as failed, then the behavior of issuing a subsequent clusterlaunchcontrol.try_cancel instruction is undefined.

### Syntax

```
clusterlaunchcontrol.try_cancel.async{.space}.completion_mechanism{.multicast::cluster::all}.b128 [addr], [mbar];

.completion_mechanism = { .mbarrier::complete_tx::bytes };
.space = { .shared::cta };
```

### Examples

```
// Assumption: 1D cluster (cluster_ctaid.y/.z == 1)
// with 1 thread per CTA.

// Current Cluster to be processed, initially the
// currently launched cluster:

mov.b32 xctaid, %ctaid.x;
barrier.cluster.arrive.relaxed;
processCluster:

// Wait on all cluster CTAs completing initialization or processing of previous cluster:

barrier.cluster.wait.acquire;
mov.u32  %r0, %tid.x;
setp.u32.eq p0, %r0, 0x0;
@!p0 bra asyncWork;

// All CTAs in the cluster arrive at their local
// SMEM   barrier and set 16B handle tx count:

mbarrier.arrive.expect_tx.cluster.relaxed.shared::cta.b64 state, [mbar], 16;

// first CTA in Cluster attempts to cancel a
// not-yet-started cluster:

mov.u32  %r0, %cluster_ctaid.x;
setp.u32.eq p0, %r0, 0x0;
@p0 clusterlaunchcontrol.try_cancel.async.mbarrier::complete_tx::bytes.multicast::cluster::all.b128 [addr], [mbar];

asyncWork:
// ...process xctaid while cancellation request completes
// asynchronously...

// All CTAs in Cluster wait on cancellation responses on their local SMEM:

waitLoop:
// .acquire prevents the load of the handle from overtaking this read:

mbarrier.try_wait.cluster.acquire.shared::cta.b64   complete, [mbar], state;
@!complete bra waitLoop;

// Load response into 16-byte wide register after unblocking
// from mbarrier:

ld.shared.b128 handle, [addr];

// Check whether cancellation succeeded:

clusterlaunchcontrol.query_cancel.is_canceled.pred.b128 p, handle;
@!p ret; // If failed, we are don end exit:

// Otherwise, read ctaid of first CTA of cancelled Cluster for next iteration...

@p clusterlaunchcontrol.query_cancel.get_first_ctaid.v4.b32.b128 {xctaid, _, _, _},  handle;

// ...and signal CTA0 that we are done reading from handle:
// Fence generic->async

fence.proxy.async.shared::cta;
barrier.cluster.arrive.relaxed;

bra processCluster;
```

