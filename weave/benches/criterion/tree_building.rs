use weave::core::Item;
use weave::Tree;
use weave::Universe;

pub fn setup_tree_20() -> Tree<Item> {
    let item0 = Item::new("0");
    let item1 = Item::new("1");
    let item2 = Item::new("2");
    let item3 = Item::new("3");
    let item4 = Item::new("4");
    let item5 = Item::new("5");
    let item6 = Item::new("6");
    let item7 = Item::new("7");
    let item8 = Item::new("8");
    let item9 = Item::new("9");
    let item10 = Item::new("10");
    let item11 = Item::new("11");
    let item12 = Item::new("12");
    let item13 = Item::new("13");
    let item14 = Item::new("14");
    let item15 = Item::new("15");
    let item16 = Item::new("16");
    let item17 = Item::new("17");
    let item18 = Item::new("18");
    let item19 = Item::new("19");

    let universe = Universe::from(vec![
        item0.clone(), item1.clone(), item2.clone(), item3.clone(), item4.clone(), item5.clone(), item6.clone(), item7.clone(), item8.clone(), item9.clone(),
        item10.clone(), item11.clone(), item12.clone(), item13.clone(), item14.clone(), item15.clone(), item16.clone(), item17.clone(), item18.clone(), item19.clone()
    ]);
    universe.hyper_tree(&[
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

pub fn setup_tree_10() -> Tree<Item> {
    let item0 = Item::new("0");
    let item1 = Item::new("1");
    let item2 = Item::new("2");
    let item3 = Item::new("3");
    let item4 = Item::new("4");
    let item5 = Item::new("5");
    let item6 = Item::new("6");
    let item7 = Item::new("7");
    let item8 = Item::new("8");
    let item9 = Item::new("9");

    let universe = Universe::from(vec![item0.clone(), item1.clone(), item2.clone(), item3.clone(), item4.clone(), item5.clone(), item6.clone(), item7.clone(), item8.clone(), item9.clone()]);
    universe.hyper_tree(&[
        vec![item0.clone(), item1.clone(), item2.clone(), item3.clone()],
        vec![item1.clone(), item2.clone(), item3.clone(), item4.clone()],
        vec![item2.clone(), item3.clone(), item4.clone(), item5.clone()],
        vec![item6.clone(), item7.clone(), item8.clone(), item9.clone()],
        vec![item1.clone(), item3.clone(), item5.clone(), item7.clone(), item9.clone()],
        vec![item0.clone(), item0.clone(), item1.clone(), item2.clone(), item3.clone(), item5.clone(), item8.clone()],
    ])
}

pub fn setup_tree_computer_parts() -> Tree<Item> {
    // CPUs

    let cpu_intel_6700k = Item::new("cpu:intel-i7-6700k");
    let cpu_intel_3770k = Item::new("cpu:intel-i7-3770k");
    let cpu_intel_4790k = Item::new("cpu:intel-i7-4790k");
    let cpu_intel_8600k = Item::new("cpu:intel-i5-8600k");
    let cpu_intel_3570k = Item::new("cpu:intel-i5-3570k");

    let cpus_intel = vec![
        cpu_intel_6700k.clone(),
        cpu_intel_3770k.clone(),
        cpu_intel_4790k.clone(),
        cpu_intel_8600k.clone(),
        cpu_intel_3570k.clone()
    ];

    let cpu_amd_2700x = Item::new("cpu:amd-ryzen-7-2700x");
    let cpu_amd_1600 = Item::new("cpu:amd-ryzen-5-1600");
    let cpu_amd_1600x = Item::new("cpu:amd-ryzen-5-1600x");
    let cpu_amd_2600 = Item::new("cpu:amd-ryzen-5-2600");

    let cpus_amd = vec![
        cpu_amd_2700x.clone(),
        cpu_amd_1600.clone(),
        cpu_amd_1600x.clone(),
        cpu_amd_2600.clone()
    ];

    // MBs

    let mb_msi_b350 = Item::new("mb:msi-b350");
    let mb_msi_z390 = Item::new("mb:msi-z390-a-pro");
    let mb_asus_b350 = Item::new("mb:asus-strix-b350-f");
    let mb_asus_z370 = Item::new("mb:asus-prime-z370-a");

    // RAM

    let ram_ripjaws_8gb = Item::new("ram:ripjaws-v-8gb");
    let ram_ripjaws_16gb = Item::new("ram:ripjaws-v-16gb");
    let ram_ripjaws_32gb = Item::new("ram:ripjaws-v-32gb");
    let ram_corsair_4gb = Item::new("ram:corsair-vengance-4gb");
    let ram_corsair_8gb = Item::new("ram:corsair-vengance-8gb");
    let ram_corsair_16gb = Item::new("ram:corsair-vengance-16gb");

    // GPUs

    let gpu_rtx_2070 = Item::new("gpu:gigabyte-geforce-rtx-2070");
    let gpu_gtx_1070i = Item::new("gpu:evga-geforce-gtx-1070i");
    let gpu_gtx_1060 = Item::new("gpu:msi-geforce-gtx-1060");

    let gpus_intel = vec![
        gpu_rtx_2070.clone(),
        gpu_gtx_1070i.clone(),
        gpu_gtx_1060.clone()
    ];

    let gpu_rx_580 = Item::new("gpu:msi-rx-580-8gb-oc");
    let gpu_rx_570 = Item::new("gpu:msi-rx-570-8gb");
    let gpu_xfx_570 = Item::new("gpu:xfx-570-8gb");

    let gpus_amd = vec![
        gpu_rx_580.clone(),
        gpu_rx_570.clone(),
        gpu_xfx_570.clone()
    ];

    // STORAGE

    let store_wd_hdd_1tb = Item::new("storage:wd-hdd-1tb");
    let store_wd_hdd_2tb = Item::new("storage:wd-hdd-2tb");
    let store_wd_hdd_4tb = Item::new("storage:wd-hdd-4tb");
    let store_wd_ssd_1tb = Item::new("storage:wd-ssd-1tb");
    let store_kingston_hdd_2tb = Item::new("storage:kingston-hdd-2tb");
    let store_kingston_hdd_4tb = Item::new("storage:kingston-hdd-4tb");
    let store_kingston_ssd_1tb = Item::new("storage:kingston-ssd-1tb");
    let store_kingston_ssd_2tb = Item::new("storage:kingston-ssd-2tb");

    // POWER

    let ps_750w = Item::new("ps:evga-750w");
    let ps_550w = Item::new("ps:corsair-550w");

    // UNIVERSE

    let universe = {
        let items: Vec<Item> = vec![
            cpus_intel,
            cpus_amd,
            vec![
                mb_msi_b350.clone(),
                mb_msi_z390.clone(),
                mb_asus_b350.clone(),
                mb_asus_z370.clone(),
            ],
            vec![
                ram_ripjaws_8gb.clone(),
                ram_ripjaws_16gb.clone(),
                ram_ripjaws_32gb.clone(),
                ram_corsair_4gb.clone(),
                ram_corsair_8gb.clone(),
                ram_corsair_16gb.clone(),
            ],
            gpus_intel,
            gpus_amd,
            vec![
                store_wd_hdd_1tb.clone(),
                store_wd_hdd_2tb.clone(),
                store_wd_hdd_4tb.clone(),
                store_wd_ssd_1tb.clone(),
                store_kingston_hdd_2tb.clone(),
                store_kingston_hdd_4tb.clone(),
                store_kingston_ssd_1tb.clone(),
                store_kingston_ssd_2tb.clone(),
            ],
            vec![
                ps_750w.clone(),
                ps_550w.clone(),
            ]
        ].into_iter().flatten().collect();

        Universe::from(items)
    };

    // CATEGORIES

    let cpus = universe.unique_tree(&[
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

    let mbs = universe.unique_tree(&[
        mb_msi_b350.clone(),
        mb_msi_z390.clone(),
        mb_asus_b350.clone(),
        mb_asus_z370.clone(),
    ]);

    let ram = universe.unique_tree(&[
        ram_ripjaws_8gb.clone(),
        ram_ripjaws_16gb.clone(),
        ram_ripjaws_32gb.clone(),
        ram_corsair_4gb.clone(),
        ram_corsair_8gb.clone(),
        ram_corsair_16gb.clone(),
    ]);

    let gpus = universe.unique_tree(&[
        gpu_rtx_2070.clone(),
        gpu_gtx_1070i.clone(),
        gpu_gtx_1060.clone(),
        gpu_rx_580.clone(),
        gpu_rx_570.clone(),
        gpu_xfx_570.clone()
    ]);

    let storage = universe.unique_tree(&[
        store_wd_hdd_1tb.clone(),
        store_wd_hdd_2tb.clone(),
        store_wd_hdd_4tb.clone(),
        store_wd_ssd_1tb.clone(),
        store_kingston_hdd_2tb.clone(),
        store_kingston_hdd_4tb.clone(),
        store_kingston_ssd_1tb.clone(),
        store_kingston_ssd_2tb.clone(),
    ]);

    let power = universe.unique_tree(&[
        ps_750w.clone(),
        ps_550w.clone(),
    ]);

    let tree = {
        let tree = Tree::product(&universe.unit_tree(), &cpus);
        let tree = Tree::product(&tree, &mbs);
        let tree = Tree::product(&tree, &ram);
        let tree = Tree::product(&tree, &gpus);
        let tree = Tree::product(&tree, &storage);
        let tree = Tree::product(&tree, &power);

        tree
    };

    tree
}