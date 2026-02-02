// Lean compiler output
// Module: ReflexiveDSHA.Basic
// Imports: public import Init public import Std
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint__with__trace(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_hash__trace(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_hash__trace___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_run__with__trace(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___redArg(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_run__with__trace___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_hash__trace___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint__with__trace___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_List_getLast_x3f___redArg(lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_hash__trace___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___redArg___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___redArg(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; uint8_t x_7; 
x_4 = lean_apply_1(x_1, x_3);
x_5 = lean_apply_1(x_2, x_4);
x_6 = lean_unsigned_to_nat(0u);
x_7 = lean_nat_dec_eq(x_5, x_6);
lean_dec(x_5);
return x_7;
}
}
LEAN_EXPORT uint8_t lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
uint8_t x_6; 
x_6 = lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___redArg(x_3, x_4, x_5);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
uint8_t x_6; lean_object* x_7; 
x_6 = lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable(x_1, x_2, x_3, x_4, x_5);
x_7 = lean_box(x_6);
return x_7;
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___redArg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
uint8_t x_4; lean_object* x_5; 
x_4 = lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___redArg(x_1, x_2, x_3);
x_5 = lean_box(x_4);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint___redArg(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
lean_inc(x_4);
lean_inc_ref(x_2);
lean_inc_ref(x_1);
x_5 = lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___redArg(x_1, x_2, x_4);
if (x_5 == 0)
{
lean_object* x_6; 
lean_inc(x_3);
x_6 = lean_apply_1(x_3, x_4);
x_4 = x_6;
goto _start;
}
else
{
lean_dec(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
return x_4;
}
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5, lean_object* x_6, lean_object* x_7) {
_start:
{
lean_object* x_8; 
x_8 = lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint___redArg(x_3, x_4, x_5, x_7);
return x_8;
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint__with__trace___redArg(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
lean_inc(x_4);
lean_inc_ref(x_2);
lean_inc_ref(x_1);
x_5 = lp_reflexive__dsha_ReflexiveDSHA_is__fixpoint__decidable___redArg(x_1, x_2, x_4);
if (x_5 == 0)
{
lean_object* x_6; lean_object* x_7; uint8_t x_8; 
lean_inc(x_3);
lean_inc(x_4);
x_6 = lean_apply_1(x_3, x_4);
x_7 = lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint__with__trace___redArg(x_1, x_2, x_3, x_6);
x_8 = !lean_is_exclusive(x_7);
if (x_8 == 0)
{
lean_object* x_9; lean_object* x_10; 
x_9 = lean_ctor_get(x_7, 1);
x_10 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_10, 0, x_4);
lean_ctor_set(x_10, 1, x_9);
lean_ctor_set(x_7, 1, x_10);
return x_7;
}
else
{
lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; 
x_11 = lean_ctor_get(x_7, 0);
x_12 = lean_ctor_get(x_7, 1);
lean_inc(x_12);
lean_inc(x_11);
lean_dec(x_7);
x_13 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_13, 0, x_4);
lean_ctor_set(x_13, 1, x_12);
x_14 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_14, 0, x_11);
lean_ctor_set(x_14, 1, x_13);
return x_14;
}
}
else
{
lean_object* x_15; lean_object* x_16; lean_object* x_17; 
lean_dec(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_15 = lean_box(0);
lean_inc(x_4);
x_16 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_16, 0, x_4);
lean_ctor_set(x_16, 1, x_15);
x_17 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_17, 0, x_4);
lean_ctor_set(x_17, 1, x_16);
return x_17;
}
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint__with__trace(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5, lean_object* x_6, lean_object* x_7) {
_start:
{
lean_object* x_8; 
x_8 = lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint__with__trace___redArg(x_3, x_4, x_5, x_7);
return x_8;
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_run__with__trace(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5, lean_object* x_6, lean_object* x_7) {
_start:
{
lean_object* x_8; 
x_8 = lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint__with__trace___redArg(x_3, x_4, x_5, x_7);
return x_8;
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_run__with__trace___redArg(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = lp_reflexive__dsha_ReflexiveDSHA_heal__to__fixpoint__with__trace___redArg(x_1, x_2, x_3, x_4);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_hash__trace___redArg(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = l_List_getLast_x3f___redArg(x_2);
if (lean_obj_tag(x_3) == 0)
{
lean_object* x_4; 
lean_dec_ref(x_1);
x_4 = lean_unsigned_to_nat(0u);
return x_4;
}
else
{
lean_object* x_5; lean_object* x_6; 
x_5 = lean_ctor_get(x_3, 0);
lean_inc(x_5);
lean_dec_ref(x_3);
x_6 = lean_apply_1(x_1, x_5);
return x_6;
}
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_hash__trace(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = lp_reflexive__dsha_ReflexiveDSHA_hash__trace___redArg(x_2, x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_hash__trace___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = lp_reflexive__dsha_ReflexiveDSHA_hash__trace(x_1, x_2, x_3);
lean_dec(x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_reflexive__dsha_ReflexiveDSHA_hash__trace___redArg___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_reflexive__dsha_ReflexiveDSHA_hash__trace___redArg(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Std(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_reflexive__dsha_ReflexiveDSHA_Basic(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Std(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
