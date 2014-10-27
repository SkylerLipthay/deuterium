
pub use from::{Table, RcTable};
pub use field::{NamedField, RcField};
pub use select_query::{SelectQuery, LimitMany};

// pub enum Value<T> {
//     ExpressionValue(T),
//     DefaultValue
// }

#[allow(dead_code)]
#[deriving(Clone)]
pub enum InsertValues<T, M> {
    InsertDefaultValues,
    InsertSelect(SelectQuery<T, LimitMany, M>)
}

#[allow(dead_code)]
#[deriving(Clone)]
pub struct InsertQuery<T, M> {
    pub into: RcTable,
    pub cols: Option<Vec<RcField>>,
    pub values: InsertValues<T, M>
}

macro_rules! insert(
    ($n:ident, $(($t:ident, $t_:ident)),+) => (
        // FIXME: Make this public after rust#17635
        fn $n<$($t,)+>(table: &Table, $($t_: &NamedField<$t>,)+) -> InsertQuery<(T), M> {
            InsertQuery::new(table)
        }
    )
)

#[allow(dead_code)]
impl<T: Clone, M: Clone> InsertQuery<T, M> {
    
    pub fn new(into: &Table) -> InsertQuery<T, M> {
        InsertQuery {
            into: into.upcast_table(),
            cols: None,
            values: InsertDefaultValues
        }
    }

    // FIXME: Make this public after rust-lang/rust#17635 and remove after rust-lang/rfcs#376
    insert!(insert_1, (T0, _t0))
    insert!(insert_2, (T0, _t0), (T1, _t1))
    insert!(insert_3, (T0, _t0), (T1, _t1), (T2, _t2))
    insert!(insert_4, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3))
    insert!(insert_5, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4))
    insert!(insert_6, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5))
    insert!(insert_7, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6))
    insert!(insert_8, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7))
    insert!(insert_9, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8))
    insert!(insert_10, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9))
    insert!(insert_11, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10))
    insert!(insert_12, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11))
    insert!(insert_13, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12))
    insert!(insert_14, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13))
    insert!(insert_15, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14))
    insert!(insert_16, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15))
    insert!(insert_17, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16))
    insert!(insert_18, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17))
    insert!(insert_19, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18))
    insert!(insert_20, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19))
    insert!(insert_21, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20))
    insert!(insert_22, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21))
    insert!(insert_23, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22))
    insert!(insert_24, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23))
    insert!(insert_25, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24))
    insert!(insert_26, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25))
    insert!(insert_27, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26))
    insert!(insert_28, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27))
    insert!(insert_29, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28))
    insert!(insert_30, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29))
    insert!(insert_31, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30))
    insert!(insert_32, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30), (T31, _t31))
    insert!(insert_33, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30), (T31, _t31), (T32, _t32))
    insert!(insert_34, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30), (T31, _t31), (T32, _t32), (T33, _t33))
    insert!(insert_35, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30), (T31, _t31), (T32, _t32), (T33, _t33), (T34, _t34))
    insert!(insert_36, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30), (T31, _t31), (T32, _t32), (T33, _t33), (T34, _t34), (T35, _t35))
    insert!(insert_37, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30), (T31, _t31), (T32, _t32), (T33, _t33), (T34, _t34), (T35, _t35), (T36, _t36))
    insert!(insert_38, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30), (T31, _t31), (T32, _t32), (T33, _t33), (T34, _t34), (T35, _t35), (T36, _t36), (T37, _t37))
    insert!(insert_39, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30), (T31, _t31), (T32, _t32), (T33, _t33), (T34, _t34), (T35, _t35), (T36, _t36), (T37, _t37), (T38, _t38))
    insert!(insert_40, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11), (T12, _t12), (T13, _t13), (T14, _t14), (T15, _t15), (T16, _t16), (T17, _t17), (T18, _t18), (T19, _t19), (T20, _t20), (T21, _t21), (T22, _t22), (T23, _t23), (T24, _t24), (T25, _t25), (T26, _t26), (T27, _t27), (T28, _t28), (T29, _t29), (T30, _t30), (T31, _t31), (T32, _t32), (T33, _t33), (T34, _t34), (T35, _t35), (T36, _t36), (T37, _t37), (T38, _t38), (T39, _t39))

}

