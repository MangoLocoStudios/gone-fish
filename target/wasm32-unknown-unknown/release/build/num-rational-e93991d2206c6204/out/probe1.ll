; ModuleID = 'probe1.75ef62f61a2a3014-cgu.0'
source_filename = "probe1.75ef62f61a2a3014-cgu.0"
target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

%"core::fmt::Arguments<'_>" = type { { ptr, i32 }, { ptr, i32 }, { ptr, i32 } }
%"alloc::string::String" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { { ptr, i32 }, i32 }
%"core::ptr::metadata::PtrRepr<[u8]>" = type { [2 x i32] }
%"alloc::alloc::Global" = type {}
%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>" = type { [1 x i32], i32, [1 x i32] }

@alloc_91c7fa63c3cfeaa3c795652d5cf060e4 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc_e90401c92a6af8b32765b1534130c461 = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_91c7fa63c3cfeaa3c795652d5cf060e4, [4 x i8] c"\0C\00\00\00" }>, align 4
@alloc_c06a172a08ac35a48b6ad59116e021fc = private unnamed_addr constant <{}> zeroinitializer, align 4
@alloc_290c72fe4b3abd3038f44a20e844c1cf = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/d627cf07ce46d230a93732a4714d16f00df9466b/library/core/src/fmt/mod.rs" }>, align 1
@alloc_94951af5144cea721ff842cb5ee1a578 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_290c72fe4b3abd3038f44a20e844c1cf, [12 x i8] c"K\00\00\00J\01\00\00\0D\00\00\00" }>, align 4
@alloc_e43ed1f48f074e1d40eca43cb2f7d955 = private unnamed_addr constant <{ [80 x i8] }> <{ [80 x i8] c"/rustc/d627cf07ce46d230a93732a4714d16f00df9466b/library/core/src/alloc/layout.rs" }>, align 1
@alloc_875a65397defc3ce507016872220ca33 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_e43ed1f48f074e1d40eca43cb2f7d955, [12 x i8] c"P\00\00\00\BF\01\00\00)\00\00\00" }>, align 4
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"
@__rust_no_alloc_shim_is_unstable = external dso_local global i8
@alloc_97350e8bf483c1fe1c3a218b02d80fb1 = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_c06a172a08ac35a48b6ad59116e021fc, [4 x i8] zeroinitializer }>, align 4
@alloc_83ea17bf0c4f4a5a5a13d3ae7955acd0 = private unnamed_addr constant <{ [4 x i8] }> zeroinitializer, align 4

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3fmt9Arguments6new_v117h0d4724aa1db72d58E(ptr sret(%"core::fmt::Arguments<'_>") align 4 %_0, ptr align 4 %pieces.0, i32 %pieces.1, ptr align 4 %args.0, i32 %args.1) unnamed_addr #0 {
start:
  %_15 = alloca { ptr, i32 }, align 4
  %_13 = alloca { ptr, i32 }, align 4
  %_11 = alloca %"core::fmt::Arguments<'_>", align 4
  %_3 = icmp ult i32 %pieces.1, %args.1
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_8 = add i32 %args.1, 1
  %_6 = icmp ugt i32 %pieces.1, %_8
  br i1 %_6, label %bb3, label %bb4

bb1:                                              ; preds = %start
  br label %bb3

bb4:                                              ; preds = %bb2
  store ptr null, ptr %_13, align 4
  %0 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 0
  store ptr %pieces.0, ptr %0, align 4
  %1 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 1
  store i32 %pieces.1, ptr %1, align 4
  %2 = getelementptr inbounds { ptr, i32 }, ptr %_13, i32 0, i32 0
  %3 = load ptr, ptr %2, align 4, !align !1, !noundef !2
  %4 = getelementptr inbounds { ptr, i32 }, ptr %_13, i32 0, i32 1
  %5 = load i32, ptr %4, align 4
  %6 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_0, i32 0, i32 2
  %7 = getelementptr inbounds { ptr, i32 }, ptr %6, i32 0, i32 0
  store ptr %3, ptr %7, align 4
  %8 = getelementptr inbounds { ptr, i32 }, ptr %6, i32 0, i32 1
  store i32 %5, ptr %8, align 4
  %9 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_0, i32 0, i32 1
  %10 = getelementptr inbounds { ptr, i32 }, ptr %9, i32 0, i32 0
  store ptr %args.0, ptr %10, align 4
  %11 = getelementptr inbounds { ptr, i32 }, ptr %9, i32 0, i32 1
  store i32 %args.1, ptr %11, align 4
  ret void

bb3:                                              ; preds = %bb1, %bb2
  store ptr null, ptr %_15, align 4
  %12 = getelementptr inbounds { ptr, i32 }, ptr %_11, i32 0, i32 0
  store ptr @alloc_e90401c92a6af8b32765b1534130c461, ptr %12, align 4
  %13 = getelementptr inbounds { ptr, i32 }, ptr %_11, i32 0, i32 1
  store i32 1, ptr %13, align 4
  %14 = getelementptr inbounds { ptr, i32 }, ptr %_15, i32 0, i32 0
  %15 = load ptr, ptr %14, align 4, !align !1, !noundef !2
  %16 = getelementptr inbounds { ptr, i32 }, ptr %_15, i32 0, i32 1
  %17 = load i32, ptr %16, align 4
  %18 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_11, i32 0, i32 2
  %19 = getelementptr inbounds { ptr, i32 }, ptr %18, i32 0, i32 0
  store ptr %15, ptr %19, align 4
  %20 = getelementptr inbounds { ptr, i32 }, ptr %18, i32 0, i32 1
  store i32 %17, ptr %20, align 4
  %21 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_11, i32 0, i32 1
  %22 = getelementptr inbounds { ptr, i32 }, ptr %21, i32 0, i32 0
  store ptr @alloc_c06a172a08ac35a48b6ad59116e021fc, ptr %22, align 4
  %23 = getelementptr inbounds { ptr, i32 }, ptr %21, i32 0, i32 1
  store i32 0, ptr %23, align 4
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h4b9f90a7f9eba302E(ptr align 4 %_11, ptr align 4 @alloc_94951af5144cea721ff842cb5ee1a578) #11
  unreachable
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint nounwind
define internal void @_ZN4core3ops8function6FnOnce9call_once17hf6033970cf250a90E(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 1 %0, i32 %1) unnamed_addr #0 {
start:
  %_2 = alloca { ptr, i32 }, align 4
  %2 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 0
  store ptr %0, ptr %2, align 4
  %3 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 1
  store i32 %1, ptr %3, align 4
  %4 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 0
  %5 = load ptr, ptr %4, align 4, !nonnull !2, !align !3, !noundef !2
  %6 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 1
  %7 = load i32, ptr %6, align 4, !noundef !2
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
  call void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17h3a301bff19f0ee81E"(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 1 %5, i32 %7) #12
  ret void
}

; core::ptr::drop_in_place<alloc::string::String>
; Function Attrs: nounwind
define hidden void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h7aa5db03e32da035E"(ptr align 4 %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hdcc22c0da8179883E"(ptr align 4 %_1) #12
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: nounwind
define hidden void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hdcc22c0da8179883E"(ptr align 4 %_1) unnamed_addr #1 {
start:
; call <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h10fd29165b1e132eE"(ptr align 4 %_1) #12
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hb678fe5aea110db1E"(ptr align 4 %_1) #12
  ret void
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: nounwind
define hidden void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hb678fe5aea110db1E"(ptr align 4 %_1) unnamed_addr #1 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb288b78f5304aa35E"(ptr align 4 %_1) #12
  ret void
}

; core::alloc::layout::Layout::array::inner
; Function Attrs: inlinehint nounwind
define internal { i32, i32 } @_ZN4core5alloc6layout6Layout5array5inner17h79d14df1608558b0E(i32 %element_size, i32 %align, i32 %n) unnamed_addr #0 {
start:
  %_18 = alloca i32, align 4
  %_13 = alloca i32, align 4
  %_9 = alloca { i32, i32 }, align 4
  %_0 = alloca { i32, i32 }, align 4
  %0 = icmp eq i32 %element_size, 0
  br i1 %0, label %bb5, label %bb1

bb5:                                              ; preds = %bb4, %start
  %array_size = mul i32 %element_size, %n
  store i32 %align, ptr %_18, align 4
  %_19 = load i32, ptr %_18, align 4, !range !4, !noundef !2
  %_20 = icmp uge i32 %_19, 1
  %_21 = icmp ule i32 %_19, -2147483648
  %_22 = and i1 %_20, %_21
  call void @llvm.assume(i1 %_22)
  %1 = getelementptr inbounds { i32, i32 }, ptr %_9, i32 0, i32 1
  store i32 %array_size, ptr %1, align 4
  store i32 %_19, ptr %_9, align 4
  %2 = getelementptr inbounds { i32, i32 }, ptr %_9, i32 0, i32 0
  %3 = load i32, ptr %2, align 4, !range !4, !noundef !2
  %4 = getelementptr inbounds { i32, i32 }, ptr %_9, i32 0, i32 1
  %5 = load i32, ptr %4, align 4, !noundef !2
  %6 = getelementptr inbounds { i32, i32 }, ptr %_0, i32 0, i32 0
  store i32 %3, ptr %6, align 4
  %7 = getelementptr inbounds { i32, i32 }, ptr %_0, i32 0, i32 1
  store i32 %5, ptr %7, align 4
  br label %bb6

bb1:                                              ; preds = %start
  store i32 %align, ptr %_13, align 4
  %_14 = load i32, ptr %_13, align 4, !range !4, !noundef !2
  %_15 = icmp uge i32 %_14, 1
  %_16 = icmp ule i32 %_14, -2147483648
  %_17 = and i1 %_15, %_16
  call void @llvm.assume(i1 %_17)
  %_11 = sub i32 %_14, 1
  %_6 = sub i32 2147483647, %_11
  %_7 = icmp eq i32 %element_size, 0
  %8 = call i1 @llvm.expect.i1(i1 %_7, i1 false)
  br i1 %8, label %panic, label %bb2

bb2:                                              ; preds = %bb1
  %_5 = udiv i32 %_6, %element_size
  %_4 = icmp ugt i32 %n, %_5
  br i1 %_4, label %bb3, label %bb4

panic:                                            ; preds = %bb1
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h1615668411ec56a8E(ptr align 1 @str.0, i32 25, ptr align 4 @alloc_875a65397defc3ce507016872220ca33) #11
  unreachable

bb4:                                              ; preds = %bb2
  br label %bb5

bb3:                                              ; preds = %bb2
  store i32 0, ptr %_0, align 4
  br label %bb6

bb6:                                              ; preds = %bb3, %bb5
  %9 = getelementptr inbounds { i32, i32 }, ptr %_0, i32 0, i32 0
  %10 = load i32, ptr %9, align 4, !range !5, !noundef !2
  %11 = getelementptr inbounds { i32, i32 }, ptr %_0, i32 0, i32 1
  %12 = load i32, ptr %11, align 4
  %13 = insertvalue { i32, i32 } poison, i32 %10, 0
  %14 = insertvalue { i32, i32 } %13, i32 %12, 1
  ret { i32, i32 } %14
}

; core::option::Option<T>::map_or_else
; Function Attrs: inlinehint nounwind
define hidden void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17h2f65f14f3354a0c2E"(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 1 %0, i32 %1, ptr align 4 %default) unnamed_addr #0 {
start:
  %_10 = alloca i8, align 1
  %_9 = alloca i8, align 1
  %_7 = alloca { ptr, i32 }, align 4
  %self = alloca { ptr, i32 }, align 4
  %2 = getelementptr inbounds { ptr, i32 }, ptr %self, i32 0, i32 0
  store ptr %0, ptr %2, align 4
  %3 = getelementptr inbounds { ptr, i32 }, ptr %self, i32 0, i32 1
  store i32 %1, ptr %3, align 4
  store i8 1, ptr %_10, align 1
  store i8 1, ptr %_9, align 1
  %4 = load ptr, ptr %self, align 4, !noundef !2
  %5 = ptrtoint ptr %4 to i32
  %6 = icmp eq i32 %5, 0
  %_4 = select i1 %6, i32 0, i32 1
  %7 = icmp eq i32 %_4, 0
  br i1 %7, label %bb1, label %bb3

bb1:                                              ; preds = %start
  store i8 0, ptr %_10, align 1
; call alloc::fmt::format::{{closure}}
  call void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17h0d64b58e6ccc31b9E"(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 4 %default) #12
  br label %bb9

bb3:                                              ; preds = %start
  %8 = getelementptr inbounds { ptr, i32 }, ptr %self, i32 0, i32 0
  %t.0 = load ptr, ptr %8, align 4, !nonnull !2, !align !3, !noundef !2
  %9 = getelementptr inbounds { ptr, i32 }, ptr %self, i32 0, i32 1
  %t.1 = load i32, ptr %9, align 4, !noundef !2
  store i8 0, ptr %_9, align 1
  %10 = getelementptr inbounds { ptr, i32 }, ptr %_7, i32 0, i32 0
  store ptr %t.0, ptr %10, align 4
  %11 = getelementptr inbounds { ptr, i32 }, ptr %_7, i32 0, i32 1
  store i32 %t.1, ptr %11, align 4
  %12 = getelementptr inbounds { ptr, i32 }, ptr %_7, i32 0, i32 0
  %13 = load ptr, ptr %12, align 4, !nonnull !2, !align !3, !noundef !2
  %14 = getelementptr inbounds { ptr, i32 }, ptr %_7, i32 0, i32 1
  %15 = load i32, ptr %14, align 4, !noundef !2
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hf6033970cf250a90E(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 1 %13, i32 %15) #12
  br label %bb9

bb9:                                              ; preds = %bb3, %bb1
  %16 = load i8, ptr %_9, align 1, !range !6, !noundef !2
  %17 = trunc i8 %16 to i1
  br i1 %17, label %bb8, label %bb6

bb6:                                              ; preds = %bb8, %bb9
  %18 = load i8, ptr %_10, align 1, !range !6, !noundef !2
  %19 = trunc i8 %18 to i1
  br i1 %19, label %bb10, label %bb7

bb8:                                              ; preds = %bb9
  br label %bb6

bb7:                                              ; preds = %bb10, %bb6
  ret void

bb10:                                             ; preds = %bb6
  br label %bb7

bb2:                                              ; No predecessors!
  unreachable
}

; alloc::fmt::format
; Function Attrs: inlinehint nounwind
define internal void @_ZN5alloc3fmt6format17hdb98493b1d75f512E(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 4 %args) unnamed_addr #0 {
start:
  %_4 = alloca ptr, align 4
  %_2 = alloca { ptr, i32 }, align 4
  %0 = getelementptr inbounds { ptr, i32 }, ptr %args, i32 0, i32 0
  %_6.0 = load ptr, ptr %0, align 4, !nonnull !2, !align !1, !noundef !2
  %1 = getelementptr inbounds { ptr, i32 }, ptr %args, i32 0, i32 1
  %_6.1 = load i32, ptr %1, align 4, !noundef !2
  %2 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %args, i32 0, i32 1
  %3 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 0
  %_7.0 = load ptr, ptr %3, align 4, !nonnull !2, !align !1, !noundef !2
  %4 = getelementptr inbounds { ptr, i32 }, ptr %2, i32 0, i32 1
  %_7.1 = load i32, ptr %4, align 4, !noundef !2
  %5 = icmp eq i32 %_6.1, 0
  br i1 %5, label %bb3, label %bb5

bb3:                                              ; preds = %start
  %6 = icmp eq i32 %_7.1, 0
  br i1 %6, label %bb7, label %bb4

bb5:                                              ; preds = %start
  %7 = icmp eq i32 %_6.1, 1
  br i1 %7, label %bb6, label %bb4

bb7:                                              ; preds = %bb3
  %8 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 0
  store ptr @alloc_c06a172a08ac35a48b6ad59116e021fc, ptr %8, align 4
  %9 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 1
  store i32 0, ptr %9, align 4
  br label %bb1

bb4:                                              ; preds = %bb6, %bb5, %bb3
  store ptr null, ptr %_2, align 4
  br label %bb1

bb1:                                              ; preds = %bb4, %bb8, %bb7
  store ptr %args, ptr %_4, align 4
  %10 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 0
  %11 = load ptr, ptr %10, align 4, !align !3, !noundef !2
  %12 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 1
  %13 = load i32, ptr %12, align 4
  %14 = load ptr, ptr %_4, align 4, !nonnull !2, !align !1, !noundef !2
; call core::option::Option<T>::map_or_else
  call void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17h2f65f14f3354a0c2E"(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 1 %11, i32 %13, ptr align 4 %14) #12
  ret void

bb6:                                              ; preds = %bb5
  %15 = icmp eq i32 %_7.1, 0
  br i1 %15, label %bb8, label %bb4

bb8:                                              ; preds = %bb6
  %s = getelementptr inbounds [0 x { ptr, i32 }], ptr %_6.0, i32 0, i32 0
  %16 = getelementptr inbounds [0 x { ptr, i32 }], ptr %_6.0, i32 0, i32 0
  %17 = getelementptr inbounds { ptr, i32 }, ptr %16, i32 0, i32 0
  %_14.0 = load ptr, ptr %17, align 4, !nonnull !2, !align !3, !noundef !2
  %18 = getelementptr inbounds { ptr, i32 }, ptr %16, i32 0, i32 1
  %_14.1 = load i32, ptr %18, align 4, !noundef !2
  %19 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 0
  store ptr %_14.0, ptr %19, align 4
  %20 = getelementptr inbounds { ptr, i32 }, ptr %_2, i32 0, i32 1
  store i32 %_14.1, ptr %20, align 4
  br label %bb1
}

; alloc::fmt::format::{{closure}}
; Function Attrs: inlinehint nounwind
define hidden void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17h0d64b58e6ccc31b9E"(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 4 %0) unnamed_addr #0 {
start:
  %_2 = alloca %"core::fmt::Arguments<'_>", align 4
  %_1 = alloca ptr, align 4
  store ptr %0, ptr %_1, align 4
  %_3 = load ptr, ptr %_1, align 4, !nonnull !2, !align !1, !noundef !2
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_2, ptr align 4 %_3, i32 24, i1 false)
; call alloc::fmt::format::format_inner
  call void @_ZN5alloc3fmt6format12format_inner17hea858a7bf22b558eE(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 4 %_2) #12
  ret void
}

; alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
; Function Attrs: inlinehint nounwind
define internal void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17h3a301bff19f0ee81E"(ptr sret(%"alloc::string::String") align 4 %_0, ptr align 1 %self.0, i32 %self.1) unnamed_addr #0 {
start:
  %v = alloca %"alloc::vec::Vec<u8>", align 4
  %bytes = alloca %"alloc::vec::Vec<u8>", align 4
; call alloc::raw_vec::RawVec<T,A>::allocate_in
  %0 = call { ptr, i32 } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17he505ead359aa238dE"(i32 %self.1, i1 zeroext false) #12
  %_12.0 = extractvalue { ptr, i32 } %0, 0
  %_12.1 = extractvalue { ptr, i32 } %0, 1
  %1 = getelementptr inbounds { ptr, i32 }, ptr %v, i32 0, i32 0
  store ptr %_12.0, ptr %1, align 4
  %2 = getelementptr inbounds { ptr, i32 }, ptr %v, i32 0, i32 1
  store i32 %_12.1, ptr %2, align 4
  %3 = getelementptr inbounds %"alloc::vec::Vec<u8>", ptr %v, i32 0, i32 1
  store i32 0, ptr %3, align 4
  %self = load ptr, ptr %v, align 4, !nonnull !2, !noundef !2
  %4 = mul i32 %self.1, 1
  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %self, ptr align 1 %self.0, i32 %4, i1 false)
  %5 = getelementptr inbounds %"alloc::vec::Vec<u8>", ptr %v, i32 0, i32 1
  store i32 %self.1, ptr %5, align 4
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %bytes, ptr align 4 %v, i32 12, i1 false)
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_0, ptr align 4 %bytes, i32 12, i1 false)
  ret void
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint nounwind
define internal { ptr, i32 } @_ZN5alloc5alloc6Global10alloc_impl17hbb49e670530bda37E(ptr align 1 %self, i32 %0, i32 %1, i1 zeroext %zeroed) unnamed_addr #0 {
start:
  %2 = alloca i8, align 1
  %_76 = alloca { ptr, i32 }, align 4
  %_75 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 4
  %_62 = alloca ptr, align 4
  %_57 = alloca i32, align 4
  %_43 = alloca i32, align 4
  %_34 = alloca { ptr, i32 }, align 4
  %_33 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 4
  %_22 = alloca i32, align 4
  %_18 = alloca { ptr, i32 }, align 4
  %self4 = alloca ptr, align 4
  %self3 = alloca ptr, align 4
  %_12 = alloca ptr, align 4
  %layout2 = alloca { i32, i32 }, align 4
  %layout1 = alloca { i32, i32 }, align 4
  %raw_ptr = alloca ptr, align 4
  %data = alloca ptr, align 4
  %_6 = alloca { ptr, i32 }, align 4
  %_0 = alloca { ptr, i32 }, align 4
  %layout = alloca { i32, i32 }, align 4
  %3 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  store i32 %0, ptr %3, align 4
  %4 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  store i32 %1, ptr %4, align 4
  %5 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %size = load i32, ptr %5, align 4, !noundef !2
  %6 = icmp eq i32 %size, 0
  br i1 %6, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %self5 = load i32, ptr %layout, align 4, !range !4, !noundef !2
  store i32 %self5, ptr %_22, align 4
  %_23 = load i32, ptr %_22, align 4, !range !4, !noundef !2
  %_24 = icmp uge i32 %_23, 1
  %_25 = icmp ule i32 %_23, -2147483648
  %_26 = and i1 %_24, %_25
  call void @llvm.assume(i1 %_26)
  %ptr = inttoptr i32 %_23 to ptr
  store ptr %ptr, ptr %data, align 4
  %_31 = load ptr, ptr %data, align 4, !noundef !2
  store ptr %_31, ptr %_34, align 4
  %7 = getelementptr inbounds { ptr, i32 }, ptr %_34, i32 0, i32 1
  store i32 0, ptr %7, align 4
  %8 = getelementptr inbounds { ptr, i32 }, ptr %_34, i32 0, i32 0
  %9 = load ptr, ptr %8, align 4, !noundef !2
  %10 = getelementptr inbounds { ptr, i32 }, ptr %_34, i32 0, i32 1
  %11 = load i32, ptr %10, align 4, !noundef !2
  %12 = getelementptr inbounds { ptr, i32 }, ptr %_33, i32 0, i32 0
  store ptr %9, ptr %12, align 4
  %13 = getelementptr inbounds { ptr, i32 }, ptr %_33, i32 0, i32 1
  store i32 %11, ptr %13, align 4
  %14 = getelementptr inbounds { ptr, i32 }, ptr %_33, i32 0, i32 0
  %ptr.0 = load ptr, ptr %14, align 4, !noundef !2
  %15 = getelementptr inbounds { ptr, i32 }, ptr %_33, i32 0, i32 1
  %ptr.1 = load i32, ptr %15, align 4, !noundef !2
  %16 = getelementptr inbounds { ptr, i32 }, ptr %_6, i32 0, i32 0
  store ptr %ptr.0, ptr %16, align 4
  %17 = getelementptr inbounds { ptr, i32 }, ptr %_6, i32 0, i32 1
  store i32 %ptr.1, ptr %17, align 4
  %18 = getelementptr inbounds { ptr, i32 }, ptr %_6, i32 0, i32 0
  %19 = load ptr, ptr %18, align 4, !nonnull !2, !noundef !2
  %20 = getelementptr inbounds { ptr, i32 }, ptr %_6, i32 0, i32 1
  %21 = load i32, ptr %20, align 4, !noundef !2
  %22 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 0
  store ptr %19, ptr %22, align 4
  %23 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 1
  store i32 %21, ptr %23, align 4
  br label %bb11

bb1:                                              ; preds = %start
  br i1 %zeroed, label %bb3, label %bb4

bb11:                                             ; preds = %bb10, %bb8, %bb2
  %24 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 0
  %25 = load ptr, ptr %24, align 4, !noundef !2
  %26 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 1
  %27 = load i32, ptr %26, align 4
  %28 = insertvalue { ptr, i32 } poison, ptr %25, 0
  %29 = insertvalue { ptr, i32 } %28, i32 %27, 1
  ret { ptr, i32 } %29

bb4:                                              ; preds = %bb1
  %30 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  %31 = load i32, ptr %30, align 4, !range !4, !noundef !2
  %32 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %33 = load i32, ptr %32, align 4, !noundef !2
  %34 = getelementptr inbounds { i32, i32 }, ptr %layout2, i32 0, i32 0
  store i32 %31, ptr %34, align 4
  %35 = getelementptr inbounds { i32, i32 }, ptr %layout2, i32 0, i32 1
  store i32 %33, ptr %35, align 4
  %36 = load volatile i8, ptr @__rust_no_alloc_shim_is_unstable, align 1
  store i8 %36, ptr %2, align 1
  %_48 = load i8, ptr %2, align 1, !noundef !2
  %37 = getelementptr inbounds { i32, i32 }, ptr %layout2, i32 0, i32 1
  %_51 = load i32, ptr %37, align 4, !noundef !2
  %self6 = load i32, ptr %layout2, align 4, !range !4, !noundef !2
  store i32 %self6, ptr %_57, align 4
  %_58 = load i32, ptr %_57, align 4, !range !4, !noundef !2
  %_59 = icmp uge i32 %_58, 1
  %_60 = icmp ule i32 %_58, -2147483648
  %_61 = and i1 %_59, %_60
  call void @llvm.assume(i1 %_61)
  %38 = call ptr @__rust_alloc(i32 %_51, i32 %_58) #12
  store ptr %38, ptr %raw_ptr, align 4
  br label %bb5

bb3:                                              ; preds = %bb1
  %39 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  %40 = load i32, ptr %39, align 4, !range !4, !noundef !2
  %41 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %42 = load i32, ptr %41, align 4, !noundef !2
  %43 = getelementptr inbounds { i32, i32 }, ptr %layout1, i32 0, i32 0
  store i32 %40, ptr %43, align 4
  %44 = getelementptr inbounds { i32, i32 }, ptr %layout1, i32 0, i32 1
  store i32 %42, ptr %44, align 4
  %45 = getelementptr inbounds { i32, i32 }, ptr %layout1, i32 0, i32 1
  %_38 = load i32, ptr %45, align 4, !noundef !2
  %self7 = load i32, ptr %layout1, align 4, !range !4, !noundef !2
  store i32 %self7, ptr %_43, align 4
  %_44 = load i32, ptr %_43, align 4, !range !4, !noundef !2
  %_45 = icmp uge i32 %_44, 1
  %_46 = icmp ule i32 %_44, -2147483648
  %_47 = and i1 %_45, %_46
  call void @llvm.assume(i1 %_47)
  %46 = call ptr @__rust_alloc_zeroed(i32 %_38, i32 %_44) #12
  store ptr %46, ptr %raw_ptr, align 4
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  %ptr8 = load ptr, ptr %raw_ptr, align 4, !noundef !2
  %_63 = ptrtoint ptr %ptr8 to i32
  %47 = icmp eq i32 %_63, 0
  br i1 %47, label %bb15, label %bb16

bb15:                                             ; preds = %bb5
  store ptr null, ptr %self4, align 4
  br label %bb6

bb16:                                             ; preds = %bb5
  store ptr %ptr8, ptr %_62, align 4
  %48 = load ptr, ptr %_62, align 4, !nonnull !2, !noundef !2
  store ptr %48, ptr %self4, align 4
  br label %bb6

bb6:                                              ; preds = %bb16, %bb15
  %49 = load ptr, ptr %self4, align 4, !noundef !2
  %50 = ptrtoint ptr %49 to i32
  %51 = icmp eq i32 %50, 0
  %_67 = select i1 %51, i32 0, i32 1
  %52 = icmp eq i32 %_67, 0
  br i1 %52, label %bb17, label %bb18

bb17:                                             ; preds = %bb6
  store ptr null, ptr %self3, align 4
  br label %bb19

bb18:                                             ; preds = %bb6
  %v = load ptr, ptr %self4, align 4, !nonnull !2, !noundef !2
  store ptr %v, ptr %self3, align 4
  br label %bb19

bb19:                                             ; preds = %bb18, %bb17
  %53 = load ptr, ptr %self3, align 4, !noundef !2
  %54 = ptrtoint ptr %53 to i32
  %55 = icmp eq i32 %54, 0
  %_69 = select i1 %55, i32 1, i32 0
  %56 = icmp eq i32 %_69, 0
  br i1 %56, label %bb21, label %bb20

bb21:                                             ; preds = %bb19
  %v9 = load ptr, ptr %self3, align 4, !nonnull !2, !noundef !2
  store ptr %v9, ptr %_12, align 4
  br label %bb7

bb20:                                             ; preds = %bb19
  store ptr null, ptr %_12, align 4
  br label %bb7

bb7:                                              ; preds = %bb20, %bb21
  %57 = load ptr, ptr %_12, align 4, !noundef !2
  %58 = ptrtoint ptr %57 to i32
  %59 = icmp eq i32 %58, 0
  %_16 = select i1 %59, i32 1, i32 0
  %60 = icmp eq i32 %_16, 0
  br i1 %60, label %bb8, label %bb10

bb8:                                              ; preds = %bb7
  %ptr10 = load ptr, ptr %_12, align 4, !nonnull !2, !noundef !2
  store ptr %ptr10, ptr %_76, align 4
  %61 = getelementptr inbounds { ptr, i32 }, ptr %_76, i32 0, i32 1
  store i32 %size, ptr %61, align 4
  %62 = getelementptr inbounds { ptr, i32 }, ptr %_76, i32 0, i32 0
  %63 = load ptr, ptr %62, align 4, !noundef !2
  %64 = getelementptr inbounds { ptr, i32 }, ptr %_76, i32 0, i32 1
  %65 = load i32, ptr %64, align 4, !noundef !2
  %66 = getelementptr inbounds { ptr, i32 }, ptr %_75, i32 0, i32 0
  store ptr %63, ptr %66, align 4
  %67 = getelementptr inbounds { ptr, i32 }, ptr %_75, i32 0, i32 1
  store i32 %65, ptr %67, align 4
  %68 = getelementptr inbounds { ptr, i32 }, ptr %_75, i32 0, i32 0
  %ptr.011 = load ptr, ptr %68, align 4, !noundef !2
  %69 = getelementptr inbounds { ptr, i32 }, ptr %_75, i32 0, i32 1
  %ptr.112 = load i32, ptr %69, align 4, !noundef !2
  %70 = getelementptr inbounds { ptr, i32 }, ptr %_18, i32 0, i32 0
  store ptr %ptr.011, ptr %70, align 4
  %71 = getelementptr inbounds { ptr, i32 }, ptr %_18, i32 0, i32 1
  store i32 %ptr.112, ptr %71, align 4
  %72 = getelementptr inbounds { ptr, i32 }, ptr %_18, i32 0, i32 0
  %73 = load ptr, ptr %72, align 4, !nonnull !2, !noundef !2
  %74 = getelementptr inbounds { ptr, i32 }, ptr %_18, i32 0, i32 1
  %75 = load i32, ptr %74, align 4, !noundef !2
  %76 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 0
  store ptr %73, ptr %76, align 4
  %77 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 1
  store i32 %75, ptr %77, align 4
  br label %bb11

bb10:                                             ; preds = %bb7
  store ptr null, ptr %_0, align 4
  br label %bb11

bb9:                                              ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVec<T,A>::allocate_in
; Function Attrs: nounwind
define hidden { ptr, i32 } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17he505ead359aa238dE"(i32 %capacity, i1 zeroext %0) unnamed_addr #1 {
start:
  %_48 = alloca ptr, align 4
  %self1 = alloca { i32, i32 }, align 4
  %_40 = alloca { i32, i32 }, align 4
  %_31 = alloca ptr, align 4
  %_30 = alloca ptr, align 4
  %self = alloca ptr, align 4
  %_26 = alloca ptr, align 4
  %result = alloca { ptr, i32 }, align 4
  %_11 = alloca { i32, i32 }, align 4
  %_7 = alloca { i32, i32 }, align 4
  %layout = alloca { i32, i32 }, align 4
  %_0 = alloca { ptr, i32 }, align 4
  %alloc = alloca %"alloc::alloc::Global", align 1
  %init = alloca i8, align 1
  %1 = zext i1 %0 to i8
  store i8 %1, ptr %init, align 1
  br i1 false, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %2 = icmp eq i32 %capacity, 0
  br i1 %2, label %bb2, label %bb3

bb2:                                              ; preds = %bb1, %start
  store ptr inttoptr (i32 1 to ptr), ptr %_31, align 4
  %3 = load ptr, ptr %_31, align 4, !nonnull !2, !noundef !2
  store ptr %3, ptr %_30, align 4
  %4 = load ptr, ptr %_30, align 4, !nonnull !2, !noundef !2
  store ptr %4, ptr %_0, align 4
  %5 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 1
  store i32 0, ptr %5, align 4
  br label %bb16

bb3:                                              ; preds = %bb1
; call core::alloc::layout::Layout::array::inner
  %6 = call { i32, i32 } @_ZN4core5alloc6layout6Layout5array5inner17h79d14df1608558b0E(i32 1, i32 1, i32 %capacity) #12
  store { i32, i32 } %6, ptr %_7, align 4
  %7 = load i32, ptr %_7, align 4, !range !5, !noundef !2
  %8 = icmp eq i32 %7, 0
  %_8 = select i1 %8, i32 1, i32 0
  %9 = icmp eq i32 %_8, 0
  br i1 %9, label %bb6, label %bb4

bb6:                                              ; preds = %bb3
  %10 = getelementptr inbounds { i32, i32 }, ptr %_7, i32 0, i32 0
  %layout.0 = load i32, ptr %10, align 4, !range !4, !noundef !2
  %11 = getelementptr inbounds { i32, i32 }, ptr %_7, i32 0, i32 1
  %layout.1 = load i32, ptr %11, align 4, !noundef !2
  %12 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  store i32 %layout.0, ptr %12, align 4
  %13 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  store i32 %layout.1, ptr %13, align 4
  %14 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %alloc_size = load i32, ptr %14, align 4, !noundef !2
  %_39 = icmp ugt i32 %alloc_size, 2147483647
  br i1 %_39, label %bb18, label %bb19

bb4:                                              ; preds = %bb3
; call alloc::raw_vec::capacity_overflow
  call void @_ZN5alloc7raw_vec17capacity_overflow17h55873903fe8b9a18E() #11
  unreachable

bb19:                                             ; preds = %bb6
  store i32 -2147483647, ptr %_11, align 4
  br label %bb20

bb18:                                             ; preds = %bb6
  store i32 0, ptr %self1, align 4
  %15 = getelementptr inbounds { i32, i32 }, ptr %self1, i32 0, i32 0
  %16 = load i32, ptr %15, align 4, !range !5, !noundef !2
  %17 = getelementptr inbounds { i32, i32 }, ptr %self1, i32 0, i32 1
  %18 = load i32, ptr %17, align 4
  %19 = getelementptr inbounds { i32, i32 }, ptr %_40, i32 0, i32 0
  store i32 %16, ptr %19, align 4
  %20 = getelementptr inbounds { i32, i32 }, ptr %_40, i32 0, i32 1
  store i32 %18, ptr %20, align 4
  %21 = getelementptr inbounds { i32, i32 }, ptr %_40, i32 0, i32 0
  %22 = load i32, ptr %21, align 4, !range !5, !noundef !2
  %23 = getelementptr inbounds { i32, i32 }, ptr %_40, i32 0, i32 1
  %24 = load i32, ptr %23, align 4
  %25 = getelementptr inbounds { i32, i32 }, ptr %_11, i32 0, i32 0
  store i32 %22, ptr %25, align 4
  %26 = getelementptr inbounds { i32, i32 }, ptr %_11, i32 0, i32 1
  store i32 %24, ptr %26, align 4
  br label %bb20

bb20:                                             ; preds = %bb18, %bb19
  %27 = load i32, ptr %_11, align 4, !range !7, !noundef !2
  %28 = icmp eq i32 %27, -2147483647
  %_14 = select i1 %28, i32 0, i32 1
  %29 = icmp eq i32 %_14, 0
  br i1 %29, label %bb8, label %bb7

bb8:                                              ; preds = %bb20
  %30 = load i8, ptr %init, align 1, !range !6, !noundef !2
  %31 = trunc i8 %30 to i1
  %_17 = zext i1 %31 to i32
  %32 = icmp eq i32 %_17, 0
  br i1 %32, label %bb10, label %bb9

bb7:                                              ; preds = %bb20
; call alloc::raw_vec::capacity_overflow
  call void @_ZN5alloc7raw_vec17capacity_overflow17h55873903fe8b9a18E() #11
  unreachable

bb10:                                             ; preds = %bb8
  %33 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  %_19.0 = load i32, ptr %33, align 4, !range !4, !noundef !2
  %34 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %_19.1 = load i32, ptr %34, align 4, !noundef !2
; call <alloc::alloc::Global as core::alloc::Allocator>::allocate
  %35 = call { ptr, i32 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h502eac593862cdf5E"(ptr align 1 %alloc, i32 %_19.0, i32 %_19.1) #12
  store { ptr, i32 } %35, ptr %result, align 4
  br label %bb13

bb9:                                              ; preds = %bb8
  %36 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  %_21.0 = load i32, ptr %36, align 4, !range !4, !noundef !2
  %37 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %_21.1 = load i32, ptr %37, align 4, !noundef !2
; call <alloc::alloc::Global as core::alloc::Allocator>::allocate_zeroed
  %38 = call { ptr, i32 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17hba59f41757e48315E"(ptr align 1 %alloc, i32 %_21.0, i32 %_21.1) #12
  store { ptr, i32 } %38, ptr %result, align 4
  br label %bb13

bb13:                                             ; preds = %bb9, %bb10
  %39 = load ptr, ptr %result, align 4, !noundef !2
  %40 = ptrtoint ptr %39 to i32
  %41 = icmp eq i32 %40, 0
  %_22 = select i1 %41, i32 1, i32 0
  %42 = icmp eq i32 %_22, 0
  br i1 %42, label %bb15, label %bb14

bb15:                                             ; preds = %bb13
  %43 = getelementptr inbounds { ptr, i32 }, ptr %result, i32 0, i32 0
  %ptr.0 = load ptr, ptr %43, align 4, !nonnull !2, !noundef !2
  %44 = getelementptr inbounds { ptr, i32 }, ptr %result, i32 0, i32 1
  %ptr.1 = load i32, ptr %44, align 4, !noundef !2
  store ptr %ptr.0, ptr %self, align 4
  %_47 = load ptr, ptr %self, align 4, !noundef !2
  store ptr %_47, ptr %_48, align 4
  %45 = load ptr, ptr %_48, align 4, !nonnull !2, !noundef !2
  store ptr %45, ptr %_26, align 4
  %46 = load ptr, ptr %_26, align 4, !nonnull !2, !noundef !2
  store ptr %46, ptr %_0, align 4
  %47 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 1
  store i32 %capacity, ptr %47, align 4
  br label %bb16

bb14:                                             ; preds = %bb13
  %48 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  %_25.0 = load i32, ptr %48, align 4, !range !4, !noundef !2
  %49 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %_25.1 = load i32, ptr %49, align 4, !noundef !2
; call alloc::alloc::handle_alloc_error
  call void @_ZN5alloc5alloc18handle_alloc_error17hf75221906dfce084E(i32 %_25.0, i32 %_25.1) #11
  unreachable

bb16:                                             ; preds = %bb2, %bb15
  %50 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 0
  %51 = load ptr, ptr %50, align 4, !nonnull !2, !noundef !2
  %52 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 1
  %53 = load i32, ptr %52, align 4, !noundef !2
  %54 = insertvalue { ptr, i32 } poison, ptr %51, 0
  %55 = insertvalue { ptr, i32 } %54, i32 %53, 1
  ret { ptr, i32 } %55

bb5:                                              ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: nounwind
define hidden void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h8656f4602d8d8fd0E"(ptr sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") align 4 %_0, ptr align 4 %self) unnamed_addr #1 {
start:
  %self2 = alloca ptr, align 4
  %self1 = alloca ptr, align 4
  %_10 = alloca ptr, align 4
  %_9 = alloca { ptr, { i32, i32 } }, align 4
  %layout = alloca { i32, i32 }, align 4
  br i1 false, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %0 = getelementptr inbounds { ptr, i32 }, ptr %self, i32 0, i32 1
  %_3 = load i32, ptr %0, align 4, !noundef !2
  %1 = icmp eq i32 %_3, 0
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %bb1, %start
  %2 = getelementptr inbounds %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", ptr %_0, i32 0, i32 1
  store i32 0, ptr %2, align 4
  br label %bb4

bb3:                                              ; preds = %bb1
  %3 = getelementptr inbounds { ptr, i32 }, ptr %self, i32 0, i32 1
  %rhs = load i32, ptr %3, align 4, !noundef !2
  %size = mul nuw i32 1, %rhs
  %4 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  store i32 %size, ptr %4, align 4
  store i32 1, ptr %layout, align 4
  %self3 = load ptr, ptr %self, align 4, !nonnull !2, !noundef !2
  store ptr %self3, ptr %self1, align 4
  %_19 = load ptr, ptr %self1, align 4, !noundef !2
  store ptr %_19, ptr %self2, align 4
  %_24 = load ptr, ptr %self2, align 4, !noundef !2
  store ptr %_24, ptr %_10, align 4
  %5 = load ptr, ptr %_10, align 4, !nonnull !2, !noundef !2
  store ptr %5, ptr %_9, align 4
  %6 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  %7 = load i32, ptr %6, align 4, !range !4, !noundef !2
  %8 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %9 = load i32, ptr %8, align 4, !noundef !2
  %10 = getelementptr inbounds { ptr, { i32, i32 } }, ptr %_9, i32 0, i32 1
  %11 = getelementptr inbounds { i32, i32 }, ptr %10, i32 0, i32 0
  store i32 %7, ptr %11, align 4
  %12 = getelementptr inbounds { i32, i32 }, ptr %10, i32 0, i32 1
  store i32 %9, ptr %12, align 4
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_0, ptr align 4 %_9, i32 12, i1 false)
  br label %bb4

bb4:                                              ; preds = %bb2, %bb3
  ret void
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint nounwind
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17ha665c376d171944eE"(ptr align 1 %self, ptr %ptr, i32 %0, i32 %1) unnamed_addr #0 {
start:
  %_14 = alloca i32, align 4
  %layout1 = alloca { i32, i32 }, align 4
  %layout = alloca { i32, i32 }, align 4
  %2 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  store i32 %0, ptr %2, align 4
  %3 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  store i32 %1, ptr %3, align 4
  %4 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %_4 = load i32, ptr %4, align 4, !noundef !2
  %5 = icmp eq i32 %_4, 0
  br i1 %5, label %bb2, label %bb1

bb2:                                              ; preds = %start
  br label %bb3

bb1:                                              ; preds = %start
  %6 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 0
  %7 = load i32, ptr %6, align 4, !range !4, !noundef !2
  %8 = getelementptr inbounds { i32, i32 }, ptr %layout, i32 0, i32 1
  %9 = load i32, ptr %8, align 4, !noundef !2
  %10 = getelementptr inbounds { i32, i32 }, ptr %layout1, i32 0, i32 0
  store i32 %7, ptr %10, align 4
  %11 = getelementptr inbounds { i32, i32 }, ptr %layout1, i32 0, i32 1
  store i32 %9, ptr %11, align 4
  %12 = getelementptr inbounds { i32, i32 }, ptr %layout1, i32 0, i32 1
  %_9 = load i32, ptr %12, align 4, !noundef !2
  %self2 = load i32, ptr %layout1, align 4, !range !4, !noundef !2
  store i32 %self2, ptr %_14, align 4
  %_15 = load i32, ptr %_14, align 4, !range !4, !noundef !2
  %_16 = icmp uge i32 %_15, 1
  %_17 = icmp ule i32 %_15, -2147483648
  %_18 = and i1 %_16, %_17
  call void @llvm.assume(i1 %_18)
  call void @__rust_dealloc(ptr %ptr, i32 %_9, i32 %_15) #12
  br label %bb3

bb3:                                              ; preds = %bb1, %bb2
  ret void
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate_zeroed
; Function Attrs: inlinehint nounwind
define internal { ptr, i32 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17hba59f41757e48315E"(ptr align 1 %self, i32 %layout.0, i32 %layout.1) unnamed_addr #0 {
start:
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i32 } @_ZN5alloc5alloc6Global10alloc_impl17hbb49e670530bda37E(ptr align 1 %self, i32 %layout.0, i32 %layout.1, i1 zeroext true) #12
  %_0.0 = extractvalue { ptr, i32 } %0, 0
  %_0.1 = extractvalue { ptr, i32 } %0, 1
  %1 = insertvalue { ptr, i32 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i32 } %1, i32 %_0.1, 1
  ret { ptr, i32 } %2
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate
; Function Attrs: inlinehint nounwind
define internal { ptr, i32 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h502eac593862cdf5E"(ptr align 1 %self, i32 %layout.0, i32 %layout.1) unnamed_addr #0 {
start:
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i32 } @_ZN5alloc5alloc6Global10alloc_impl17hbb49e670530bda37E(ptr align 1 %self, i32 %layout.0, i32 %layout.1, i1 zeroext false) #12
  %_0.0 = extractvalue { ptr, i32 } %0, 0
  %_0.1 = extractvalue { ptr, i32 } %0, 1
  %1 = insertvalue { ptr, i32 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i32 } %1, i32 %_0.1, 1
  ret { ptr, i32 } %2
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nounwind
define hidden void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h10fd29165b1e132eE"(ptr align 4 %self) unnamed_addr #1 {
start:
  %_10 = alloca { ptr, i32 }, align 4
  %_9 = alloca %"core::ptr::metadata::PtrRepr<[u8]>", align 4
  %self1 = load ptr, ptr %self, align 4, !nonnull !2, !noundef !2
  %0 = getelementptr inbounds %"alloc::vec::Vec<u8>", ptr %self, i32 0, i32 1
  %len = load i32, ptr %0, align 4, !noundef !2
  store ptr %self1, ptr %_10, align 4
  %1 = getelementptr inbounds { ptr, i32 }, ptr %_10, i32 0, i32 1
  store i32 %len, ptr %1, align 4
  %2 = getelementptr inbounds { ptr, i32 }, ptr %_10, i32 0, i32 0
  %3 = load ptr, ptr %2, align 4, !noundef !2
  %4 = getelementptr inbounds { ptr, i32 }, ptr %_10, i32 0, i32 1
  %5 = load i32, ptr %4, align 4, !noundef !2
  %6 = getelementptr inbounds { ptr, i32 }, ptr %_9, i32 0, i32 0
  store ptr %3, ptr %6, align 4
  %7 = getelementptr inbounds { ptr, i32 }, ptr %_9, i32 0, i32 1
  store i32 %5, ptr %7, align 4
  %8 = getelementptr inbounds { ptr, i32 }, ptr %_9, i32 0, i32 0
  %_2.0 = load ptr, ptr %8, align 4, !noundef !2
  %9 = getelementptr inbounds { ptr, i32 }, ptr %_9, i32 0, i32 1
  %_2.1 = load i32, ptr %9, align 4, !noundef !2
  ret void
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: nounwind
define hidden void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb288b78f5304aa35E"(ptr align 4 %self) unnamed_addr #1 {
start:
  %_2 = alloca %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", align 4
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h8656f4602d8d8fd0E"(ptr sret(%"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>") align 4 %_2, ptr align 4 %self) #12
  %0 = getelementptr inbounds %"core::option::Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", ptr %_2, i32 0, i32 1
  %1 = load i32, ptr %0, align 4, !range !5, !noundef !2
  %2 = icmp eq i32 %1, 0
  %_4 = select i1 %2, i32 0, i32 1
  %3 = icmp eq i32 %_4, 1
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %start
  %ptr = load ptr, ptr %_2, align 4, !nonnull !2, !noundef !2
  %4 = getelementptr inbounds { ptr, { i32, i32 } }, ptr %_2, i32 0, i32 1
  %5 = getelementptr inbounds { i32, i32 }, ptr %4, i32 0, i32 0
  %layout.0 = load i32, ptr %5, align 4, !range !4, !noundef !2
  %6 = getelementptr inbounds { i32, i32 }, ptr %4, i32 0, i32 1
  %layout.1 = load i32, ptr %6, align 4, !noundef !2
  %_7 = getelementptr i8, ptr %self, i32 8
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17ha665c376d171944eE"(ptr align 1 %_7, ptr %ptr, i32 %layout.0, i32 %layout.1) #12
  br label %bb4

bb4:                                              ; preds = %bb2, %start
  ret void
}

; probe1::probe
; Function Attrs: nounwind
define hidden void @_ZN6probe15probe17h7bf1dd0aa9e974b2E() unnamed_addr #1 {
start:
  %_0.i = alloca { ptr, ptr }, align 4
  %_7 = alloca [1 x { ptr, ptr }], align 4
  %_3 = alloca %"core::fmt::Arguments<'_>", align 4
  %res = alloca %"alloc::string::String", align 4
  %_1 = alloca %"alloc::string::String", align 4
  store ptr @alloc_83ea17bf0c4f4a5a5a13d3ae7955acd0, ptr %_0.i, align 4
  %0 = getelementptr inbounds { ptr, ptr }, ptr %_0.i, i32 0, i32 1
  store ptr @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17hbc9a0b32ff12a747E", ptr %0, align 4
  %1 = load ptr, ptr %_0.i, align 4, !nonnull !2, !align !3, !noundef !2
  %2 = getelementptr inbounds { ptr, ptr }, ptr %_0.i, i32 0, i32 1
  %3 = load ptr, ptr %2, align 4, !nonnull !2, !noundef !2
  %4 = insertvalue { ptr, ptr } poison, ptr %1, 0
  %5 = insertvalue { ptr, ptr } %4, ptr %3, 1
  %_8.0 = extractvalue { ptr, ptr } %5, 0
  %_8.1 = extractvalue { ptr, ptr } %5, 1
  %6 = getelementptr inbounds [1 x { ptr, ptr }], ptr %_7, i32 0, i32 0
  %7 = getelementptr inbounds { ptr, ptr }, ptr %6, i32 0, i32 0
  store ptr %_8.0, ptr %7, align 4
  %8 = getelementptr inbounds { ptr, ptr }, ptr %6, i32 0, i32 1
  store ptr %_8.1, ptr %8, align 4
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h0d4724aa1db72d58E(ptr sret(%"core::fmt::Arguments<'_>") align 4 %_3, ptr align 4 @alloc_97350e8bf483c1fe1c3a218b02d80fb1, i32 1, ptr align 4 %_7, i32 1) #12
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17hdb98493b1d75f512E(ptr sret(%"alloc::string::String") align 4 %res, ptr align 4 %_3) #12
  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_1, ptr align 4 %res, i32 12, i1 false)
; call core::ptr::drop_in_place<alloc::string::String>
  call void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h7aa5db03e32da035E"(ptr align 4 %_1) #12
  ret void
}

; core::fmt::num::imp::<impl core::fmt::LowerExp for isize>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num3imp55_$LT$impl$u20$core..fmt..LowerExp$u20$for$u20$isize$GT$3fmt17hbc9a0b32ff12a747E"(ptr align 4, ptr align 4) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking9panic_fmt17h4b9f90a7f9eba302E(ptr align 4, ptr align 4) unnamed_addr #2

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare hidden void @llvm.assume(i1 noundef) #3

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare hidden i1 @llvm.expect.i1(i1, i1) #4

; core::panicking::panic
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core9panicking5panic17h1615668411ec56a8E(ptr align 1, i32, ptr align 4) unnamed_addr #2

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i32(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i32, i1 immarg) #5

; alloc::fmt::format::format_inner
; Function Attrs: nounwind
declare dso_local void @_ZN5alloc3fmt6format12format_inner17hea858a7bf22b558eE(ptr sret(%"alloc::string::String") align 4, ptr align 4) unnamed_addr #1

; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0)
declare dso_local noalias ptr @__rust_alloc(i32, i32 allocalign) unnamed_addr #6

; Function Attrs: nounwind allockind("alloc,zeroed,aligned") allocsize(0)
declare dso_local noalias ptr @__rust_alloc_zeroed(i32, i32 allocalign) unnamed_addr #7

; alloc::alloc::handle_alloc_error
; Function Attrs: cold noreturn nounwind
declare dso_local void @_ZN5alloc5alloc18handle_alloc_error17hf75221906dfce084E(i32, i32) unnamed_addr #8

; alloc::raw_vec::capacity_overflow
; Function Attrs: noreturn nounwind
declare dso_local void @_ZN5alloc7raw_vec17capacity_overflow17h55873903fe8b9a18E() unnamed_addr #9

; Function Attrs: nounwind allockind("free")
declare dso_local void @__rust_dealloc(ptr allocptr, i32, i32) unnamed_addr #10

attributes #0 = { inlinehint nounwind "target-cpu"="generic" }
attributes #1 = { nounwind "target-cpu"="generic" }
attributes #2 = { cold noinline noreturn nounwind "target-cpu"="generic" }
attributes #3 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #4 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #5 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #6 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) "alloc-family"="__rust_alloc" "target-cpu"="generic" }
attributes #7 = { nounwind allockind("alloc,zeroed,aligned") allocsize(0) "alloc-family"="__rust_alloc" "target-cpu"="generic" }
attributes #8 = { cold noreturn nounwind "target-cpu"="generic" }
attributes #9 = { noreturn nounwind "target-cpu"="generic" }
attributes #10 = { nounwind allockind("free") "alloc-family"="__rust_alloc" "target-cpu"="generic" }
attributes #11 = { noreturn nounwind }
attributes #12 = { nounwind }

!llvm.ident = !{!0}

!0 = !{!"rustc version 1.75.0-nightly (d627cf07c 2023-10-10)"}
!1 = !{i64 4}
!2 = !{}
!3 = !{i64 1}
!4 = !{i32 1, i32 -2147483647}
!5 = !{i32 0, i32 -2147483647}
!6 = !{i8 0, i8 2}
!7 = !{i32 0, i32 -2147483646}
