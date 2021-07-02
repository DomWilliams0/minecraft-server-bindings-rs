mod types;

macro_rules! version {
    ($module:ident, $feature:expr) => {
        #[cfg(feature = $feature)]
        pub mod $module;
    };
}

version!(v1_17, "v1-17");
version!(v21w07a, "v21w07a");
// version!(v1_16_4, "v1-16-4");
// version!(v1_16_3, "v1-16-3");
// version!(v1_16_2, "v1-16-2");
// version!(v1_16_1, "v1-16-1");
// version!(v1_16, "v1-16");
// version!(v1_16_rc1, "v1-16-rc1");
// version!(v20w14a, "v20w14a");
// version!(v20w13b, "v20w13b");
// version!(v1_15_2, "v1-15-2");
// version!(v1_15_1, "v1-15-1");
// version!(v1_15, "v1-15");
// version!(v1_14_4, "v1-14-4");
// version!(v1_14_3, "v1-14-3");
// version!(v1_14_1, "v1-14-1");
// version!(v1_14, "v1-14");
// version!(v1_13_2, "v1-13-2");
// version!(v1_13_2_pre2, "v1-13-2-pre2");
// version!(v1_13_2_pre1, "v1-13-2-pre1");
// version!(v1_13_1, "v1-13-1");
// version!(v1_13, "v1-13");
// version!(v17w50a, "v17w50a");
// version!(v1_12_2, "v1-12-2");
// version!(v1_12_1, "v1-12-1");
// version!(v1_12, "v1-12");
// version!(v1_12_pre4, "v1-12-pre4");
// version!(v17w18b, "v17w18b");
// version!(v17w15a, "v17w15a");
// version!(v1_11_2, "v1-11-2");
version!(v1_11, "v1-11");
version!(v16w35a, "v16w35a");
// version!(v1_10_2, "v1-10-2");
// version!(v1_10_1, "v1-10-1");
version!(v1_10, "v1-10");
version!(v1_10_pre1, "v1-10-pre1");
version!(v16w20a, "v16w20a");
version!(v1_9_4, "v1-9-4");
version!(v1_9_2, "v1-9-2");
version!(v1_9_1_pre2, "v1-9-1-pre2");
version!(v1_9, "v1-9");
version!(v15w40b, "v15w40b");
// version!(v1_8, "v1-8");
// version!(v1_7, "v1-7");
// version!(v0_30c, "v0-30c");
