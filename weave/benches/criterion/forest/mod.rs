use weave::Forest;

pub use self::trees::bench_trees;

mod trees;

pub fn forest_10<'a, F: Forest<&'a str>>() -> F {
    let item0 = "0";
    let item1 = "1";
    let item2 = "2";
    let item3 = "3";
    let item4 = "4";
    let item5 = "5";
    let item6 = "6";
    let item7 = "7";
    let item8 = "8";
    let item9 = "9";

    F::many(&[
        vec![item0, item1, item2, item3],
        vec![item1, item2, item3, item4],
        vec![item2, item3, item4, item5],
        vec![item6, item7, item8, item9],
        vec![item1, item3, item5, item7, item9],
        vec![item0, item0, item1, item2, item3, item5, item8],
    ])
}

pub fn forest_20<'a, F: Forest<&'a str>>() -> F {
    let item0 = "0";
    let item1 = "1";
    let item2 = "2";
    let item3 = "3";
    let item4 = "4";
    let item5 = "5";
    let item6 = "6";
    let item7 = "7";
    let item8 = "8";
    let item9 = "9";
    let item10 = "10";
    let item11 = "11";
    let item12 = "12";
    let item13 = "13";
    let item14 = "14";
    let item15 = "15";
    let item16 = "16";
    let item17 = "17";
    let item18 = "18";
    let item19 = "19";

    F::many(&[
        vec![item0, item1, item2, item3],
        vec![item1, item2, item3, item4],
        vec![item2, item3, item4, item5],
        vec![item6, item7, item8, item9],
        vec![item1, item3, item5, item7, item9],
        vec![item0, item0, item1, item2, item3, item5, item8],
        vec![item10, item11, item12, item13],
        vec![item11, item12, item13, item14],
        vec![item12, item13, item14, item15],
        vec![item16, item17, item18, item19],
        vec![item11, item13, item15, item17, item19],
        vec![item10, item10, item11, item12, item13, item15, item18],
        vec![item0, item1, item2, item13],
        vec![item1, item2, item3, item14],
        vec![item12, item13, item4, item5],
    ])
}

pub fn computer_parts<'a, F: Forest<&'a str>>() -> F {
    // CPUs

    let cpu_intel_6700k = "cpu:intel-i7-6700k";
    let cpu_intel_3770k = "cpu:intel-i7-3770k";
    let cpu_intel_4790k = "cpu:intel-i7-4790k";
    let cpu_intel_8600k = "cpu:intel-i5-8600k";
    let cpu_intel_3570k = "cpu:intel-i5-3570k";

    let _cpus_intel = vec![
        cpu_intel_6700k,
        cpu_intel_3770k,
        cpu_intel_4790k,
        cpu_intel_8600k,
        cpu_intel_3570k
    ];

    let cpu_amd_2700x = "cpu:amd-ryzen-7-2700x";
    let cpu_amd_1600 = "cpu:amd-ryzen-5-1600";
    let cpu_amd_1600x = "cpu:amd-ryzen-5-1600x";
    let cpu_amd_2600 = "cpu:amd-ryzen-5-2600";

    let _cpus_amd = vec![
        cpu_amd_2700x,
        cpu_amd_1600,
        cpu_amd_1600x,
        cpu_amd_2600
    ];

    // MBs

    let mb_msi_b350 = "mb:msi-b350";
    let mb_msi_z390 = "mb:msi-z390-a-pro";
    let mb_asus_b350 = "mb:asus-strix-b350-f";
    let mb_asus_z370 = "mb:asus-prime-z370-a";

    // RAM

    let ram_ripjaws_8gb = "ram:ripjaws-v-8gb";
    let ram_ripjaws_16gb = "ram:ripjaws-v-16gb";
    let ram_ripjaws_32gb = "ram:ripjaws-v-32gb";
    let ram_corsair_4gb = "ram:corsair-vengance-4gb";
    let ram_corsair_8gb = "ram:corsair-vengance-8gb";
    let ram_corsair_16gb = "ram:corsair-vengance-16gb";

    // GPUs

    let gpu_rtx_2070 = "gpu:gigabyte-geforce-rtx-2070";
    let gpu_gtx_1070i = "gpu:evga-geforce-gtx-1070i";
    let gpu_gtx_1060 = "gpu:msi-geforce-gtx-1060";

    let _gpus_intel = vec![
        gpu_rtx_2070,
        gpu_gtx_1070i,
        gpu_gtx_1060
    ];

    let gpu_rx_580 = "gpu:msi-rx-580-8gb-oc";
    let gpu_rx_570 = "gpu:msi-rx-570-8gb";
    let gpu_xfx_570 = "gpu:xfx-570-8gb";

    let _gpus_amd = vec![
        gpu_rx_580,
        gpu_rx_570,
        gpu_xfx_570
    ];

    // STORAGE

    let store_wd_hdd_1tb = "storage:wd-hdd-1tb";
    let store_wd_hdd_2tb = "storage:wd-hdd-2tb";
    let store_wd_hdd_4tb = "storage:wd-hdd-4tb";
    let store_wd_ssd_1tb = "storage:wd-ssd-1tb";
    let store_kingston_hdd_2tb = "storage:kingston-hdd-2tb";
    let store_kingston_hdd_4tb = "storage:kingston-hdd-4tb";
    let store_kingston_ssd_1tb = "storage:kingston-ssd-1tb";
    let store_kingston_ssd_2tb = "storage:kingston-ssd-2tb";

    // POWER

    let ps_750w = "ps:evga-750w";
    let ps_550w = "ps:corsair-550w";


    // CATEGORIES

    let cpus = F::unique(&[
        cpu_intel_6700k,
        cpu_intel_3770k,
        cpu_intel_4790k,
        cpu_intel_8600k,
        cpu_intel_3570k,
        cpu_amd_2700x,
        cpu_amd_1600,
        cpu_amd_1600x,
        cpu_amd_2600
    ]);

    let mbs = F::unique(&[
        mb_msi_b350,
        mb_msi_z390,
        mb_asus_b350,
        mb_asus_z370,
    ]);

    let ram = F::unique(&[
        ram_ripjaws_8gb,
        ram_ripjaws_16gb,
        ram_ripjaws_32gb,
        ram_corsair_4gb,
        ram_corsair_8gb,
        ram_corsair_16gb,
    ]);

    let gpus = F::unique(&[
        gpu_rtx_2070,
        gpu_gtx_1070i,
        gpu_gtx_1060,
        gpu_rx_580,
        gpu_rx_570,
        gpu_xfx_570
    ]);

    let storage = F::unique(&[
        store_wd_hdd_1tb,
        store_wd_hdd_2tb,
        store_wd_hdd_4tb,
        store_wd_ssd_1tb,
        store_kingston_hdd_2tb,
        store_kingston_hdd_4tb,
        store_kingston_ssd_1tb,
        store_kingston_ssd_2tb,
    ]);

    let power = F::unique(&[
        ps_750w,
        ps_550w,
    ]);

    let tree = {
        let tree = cpus;
        let tree = F::product(tree, mbs);
        let tree = F::product(tree, ram);
        let tree = F::product(tree, gpus);
        let tree = F::product(tree, storage);
        let tree = F::product(tree, power);

        tree
    };

    tree
}
