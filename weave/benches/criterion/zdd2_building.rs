use weave::zdd2::Forest;

pub fn setup_forest_20() -> Forest<&'static str> {
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

    Forest::many(&[
        vec![item0.clone(), item1.clone(), item2.clone(), item3.clone()],
        vec![item1.clone(), item2.clone(), item3.clone(), item4.clone()],
        vec![item2.clone(), item3.clone(), item4.clone(), item5.clone()],
        vec![item6.clone(), item7.clone(), item8.clone(), item9.clone()],
        vec![item1.clone(), item3.clone(), item5.clone(), item7.clone(), item9.clone()],
        vec![item0.clone(), item0.clone(), item1.clone(), item2.clone(), item3.clone(), item5.clone(), item8.clone()],
        vec![item10.clone(), item11.clone(), item12.clone(), item13.clone()],
        vec![item11.clone(), item12.clone(), item13.clone(), item14.clone()],
        vec![item12.clone(), item13.clone(), item14.clone(), item15.clone()],
        vec![item16.clone(), item17.clone(), item18.clone(), item19.clone()],
        vec![item11.clone(), item13.clone(), item15.clone(), item17.clone(), item19.clone()],
        vec![item10.clone(), item10.clone(), item11.clone(), item12.clone(), item13.clone(), item15.clone(), item18.clone()],
        vec![item0.clone(), item1.clone(), item2.clone(), item13.clone()],
        vec![item1.clone(), item2.clone(), item3.clone(), item14.clone()],
        vec![item12.clone(), item13.clone(), item4.clone(), item5.clone()],
    ])
}

pub fn setup_forest_10() -> Forest<&'static str> {
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

    Forest::many(&[
        vec![item0.clone(), item1.clone(), item2.clone(), item3.clone()],
        vec![item1.clone(), item2.clone(), item3.clone(), item4.clone()],
        vec![item2.clone(), item3.clone(), item4.clone(), item5.clone()],
        vec![item6.clone(), item7.clone(), item8.clone(), item9.clone()],
        vec![item1.clone(), item3.clone(), item5.clone(), item7.clone(), item9.clone()],
        vec![item0.clone(), item0.clone(), item1.clone(), item2.clone(), item3.clone(), item5.clone(), item8.clone()],
    ])
}

pub fn setup_forest_computer_parts() -> Forest<&'static str> {
    // CPUs

    let cpu_intel_6700k = "cpu:intel-i7-6700k";
    let cpu_intel_3770k = "cpu:intel-i7-3770k";
    let cpu_intel_4790k = "cpu:intel-i7-4790k";
    let cpu_intel_8600k = "cpu:intel-i5-8600k";
    let cpu_intel_3570k = "cpu:intel-i5-3570k";

    let cpus_intel = vec![
        cpu_intel_6700k.clone(),
        cpu_intel_3770k.clone(),
        cpu_intel_4790k.clone(),
        cpu_intel_8600k.clone(),
        cpu_intel_3570k.clone()
    ];

    let cpu_amd_2700x = "cpu:amd-ryzen-7-2700x";
    let cpu_amd_1600 = "cpu:amd-ryzen-5-1600";
    let cpu_amd_1600x = "cpu:amd-ryzen-5-1600x";
    let cpu_amd_2600 = "cpu:amd-ryzen-5-2600";

    let cpus_amd = vec![
        cpu_amd_2700x.clone(),
        cpu_amd_1600.clone(),
        cpu_amd_1600x.clone(),
        cpu_amd_2600.clone()
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

    let gpus_intel = vec![
        gpu_rtx_2070.clone(),
        gpu_gtx_1070i.clone(),
        gpu_gtx_1060.clone()
    ];

    let gpu_rx_580 = "gpu:msi-rx-580-8gb-oc";
    let gpu_rx_570 = "gpu:msi-rx-570-8gb";
    let gpu_xfx_570 = "gpu:xfx-570-8gb";

    let gpus_amd = vec![
        gpu_rx_580.clone(),
        gpu_rx_570.clone(),
        gpu_xfx_570.clone()
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

    let cpus = Forest::unique(&[
        cpu_intel_6700k.clone(),
        cpu_intel_3770k.clone(),
        cpu_intel_4790k.clone(),
        cpu_intel_8600k.clone(),
        cpu_intel_3570k.clone(),
        cpu_amd_2700x.clone(),
        cpu_amd_1600.clone(),
        cpu_amd_1600x.clone(),
        cpu_amd_2600.clone()
    ]);

    let mbs = Forest::unique(&[
        mb_msi_b350.clone(),
        mb_msi_z390.clone(),
        mb_asus_b350.clone(),
        mb_asus_z370.clone(),
    ]);

    let ram = Forest::unique(&[
        ram_ripjaws_8gb.clone(),
        ram_ripjaws_16gb.clone(),
        ram_ripjaws_32gb.clone(),
        ram_corsair_4gb.clone(),
        ram_corsair_8gb.clone(),
        ram_corsair_16gb.clone(),
    ]);

    let gpus = Forest::unique(&[
        gpu_rtx_2070.clone(),
        gpu_gtx_1070i.clone(),
        gpu_gtx_1060.clone(),
        gpu_rx_580.clone(),
        gpu_rx_570.clone(),
        gpu_xfx_570.clone()
    ]);

    let storage = Forest::unique(&[
        store_wd_hdd_1tb.clone(),
        store_wd_hdd_2tb.clone(),
        store_wd_hdd_4tb.clone(),
        store_wd_ssd_1tb.clone(),
        store_kingston_hdd_2tb.clone(),
        store_kingston_hdd_4tb.clone(),
        store_kingston_ssd_1tb.clone(),
        store_kingston_ssd_2tb.clone(),
    ]);

    let power = Forest::unique(&[
        ps_750w.clone(),
        ps_550w.clone(),
    ]);

    let tree = {
        let tree = cpus;
        let tree = Forest::product(tree, mbs);
        let tree = Forest::product(tree, ram);
        let tree = Forest::product(tree, gpus);
        let tree = Forest::product(tree, storage);
        let tree = Forest::product(tree, power);

        tree
    };

    tree
}
