; ModuleID = 'probe1.3a1fbbbh-cgu.0'
source_filename = "probe1.3a1fbbbh-cgu.0"
target datalayout = "e-m:x-p:32:32-i64:64-f80:32-n8:16:32-a:0:32-S32"
target triple = "i686-pc-windows-msvc"

%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>" = type { [0 x i32], %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", [0 x i32] }
%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>" = type { [0 x i32], { i32, i32 }, [0 x i32], i32, [0 x i8], i8, [3 x i8] }
%"core::panic::Location" = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }

@0 = private unnamed_addr constant <{ [27 x i8] }> <{ [27 x i8] c"assertion failed: step != 0" }>, align 1
@1 = private unnamed_addr constant <{ [73 x i8] }> <{ [73 x i8] c"/rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8\5Csrc\5Clibcore\5Cmacros\5Cmod.rs" }>, align 1
@2 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [73 x i8] }>, <{ [73 x i8] }>* @1, i32 0, i32 0, i32 0), [12 x i8] c"I\00\00\00\0F\00\00\00(\00\00\00" }>, align 4

; core::iter::traits::iterator::Iterator::rev
; Function Attrs: inlinehint uwtable
define void @_ZN4core4iter6traits8iterator8Iterator3rev17hd89a3d5eefbce3e7E(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(16), %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(16) %self) unnamed_addr #0 {
start:
  %_2 = alloca %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", align 4
  %1 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %self to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 16, i1 false)
; call core::iter::adapters::Rev<T>::new
  call void @"_ZN4core4iter8adapters12Rev$LT$T$GT$3new17hf4662b11a6444e1bE"(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %0, %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(16) %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::traits::iterator::Iterator::step_by
; Function Attrs: inlinehint uwtable
define void @_ZN4core4iter6traits8iterator8Iterator7step_by17he8662b03b3f983c6E(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(16), i32 %self.0, i32 %self.1, i32 %step) unnamed_addr #0 {
start:
; call core::iter::adapters::StepBy<I>::new
  call void @"_ZN4core4iter8adapters15StepBy$LT$I$GT$3new17hf866b7d8ea8e6b1cE"(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(16) %0, i32 %self.0, i32 %self.1, i32 %step)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::adapters::Rev<T>::new
; Function Attrs: uwtable
define void @"_ZN4core4iter8adapters12Rev$LT$T$GT$3new17hf4662b11a6444e1bE"(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(16), %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(16) %iter) unnamed_addr #1 {
start:
  %_2 = alloca %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", align 4
  %1 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %iter to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 16, i1 false)
  %3 = bitcast %"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* %0 to %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"*
  %4 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %3 to i8*
  %5 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %4, i8* align 4 %5, i32 16, i1 false)
  ret void
}

; core::iter::adapters::StepBy<I>::new
; Function Attrs: uwtable
define void @"_ZN4core4iter8adapters15StepBy$LT$I$GT$3new17hf866b7d8ea8e6b1cE"(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(16), i32 %iter.0, i32 %iter.1, i32 %step) unnamed_addr #1 personality i32 (...)* @__CxxFrameHandler3 {
start:
  %1 = alloca %"core::panic::Location"*, align 4
  %_4 = icmp ne i32 %step, 0
  %_3 = xor i1 %_4, true
  br i1 %_3, label %bb3, label %bb2

bb1:                                              ; preds = %bb5
  cleanupret from %cleanuppad unwind to caller

bb2:                                              ; preds = %start
  %_11 = sub i32 %step, 1
  %2 = bitcast %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %0 to { i32, i32 }*
  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %2, i32 0, i32 0
  store i32 %iter.0, i32* %3, align 4
  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %2, i32 0, i32 1
  store i32 %iter.1, i32* %4, align 4
  %5 = getelementptr inbounds %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %0, i32 0, i32 3
  store i32 %_11, i32* %5, align 4
  %6 = getelementptr inbounds %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* %0, i32 0, i32 5
  store i8 1, i8* %6, align 4
  ret void

bb3:                                              ; preds = %start
  store %"core::panic::Location"* bitcast (<{ i8*, [12 x i8] }>* @2 to %"core::panic::Location"*), %"core::panic::Location"** %1, align 4
  %_8 = load %"core::panic::Location"*, %"core::panic::Location"** %1, align 4, !nonnull !1
  br label %bb4

bb4:                                              ; preds = %bb3
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17h04ef46973ca54ab2E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [27 x i8] }>* @0 to [0 x i8]*), i32 27, %"core::panic::Location"* noalias readonly align 4 dereferenceable(16) %_8)
          to label %unreachable unwind label %funclet_bb5

bb5:                                              ; preds = %funclet_bb5
  br label %bb1

funclet_bb5:                                      ; preds = %bb4
  %cleanuppad = cleanuppad within none []
  br label %bb5

unreachable:                                      ; preds = %bb4
  unreachable
}

; probe1::probe
; Function Attrs: uwtable
define void @_ZN6probe15probe17he203a3b61d9abeacE() unnamed_addr #1 {
start:
  %_3 = alloca { i32, i32 }, align 4
  %_2 = alloca %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>", align 4
  %_1 = alloca %"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>", align 4
  %0 = bitcast { i32, i32 }* %_3 to i32*
  store i32 0, i32* %0, align 4
  %1 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  store i32 10, i32* %1, align 4
  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 0
  %3 = load i32, i32* %2, align 4
  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  %5 = load i32, i32* %4, align 4
; call core::iter::traits::iterator::Iterator::step_by
  call void @_ZN4core4iter6traits8iterator8Iterator7step_by17he8662b03b3f983c6E(%"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture sret dereferenceable(16) %_2, i32 %3, i32 %5, i32 2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::iter::traits::iterator::Iterator::rev
  call void @_ZN4core4iter6traits8iterator8Iterator3rev17hd89a3d5eefbce3e7E(%"core::iter::adapters::Rev<core::iter::adapters::StepBy<core::ops::range::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %_1, %"core::iter::adapters::StepBy<core::ops::range::Range<i32>>"* noalias nocapture dereferenceable(16) %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i32(i8* nocapture writeonly, i8* nocapture readonly, i32, i1 immarg) #2

declare i32 @__CxxFrameHandler3(...) unnamed_addr #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h04ef46973ca54ab2E([0 x i8]* noalias nonnull readonly align 1, i32, %"core::panic::Location"* noalias readonly align 4 dereferenceable(16)) unnamed_addr #4

attributes #0 = { inlinehint uwtable "target-cpu"="pentium4" }
attributes #1 = { uwtable "target-cpu"="pentium4" }
attributes #2 = { argmemonly nounwind }
attributes #3 = { "target-cpu"="pentium4" }
attributes #4 = { cold noinline noreturn uwtable "target-cpu"="pentium4" }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{}
