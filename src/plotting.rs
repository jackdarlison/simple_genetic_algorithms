#[macro_export]
macro_rules! plot_bar {
    ($title:expr, $x_data:expr, $y_data:expr) => {
        let output_dir = format!("out/{}.png", $title.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect::<String>());
        let root_area = BitMapBackend::new(&output_dir, (600, 400)).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        let max_y = *$y_data.iter().max().expect("Y data should have at least one value");
        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption($title, ("sans-serif", 20))
            .build_cartesian_2d($x_data.into_segmented(), 0..(max_y + (max_y / 10)))
            .unwrap();

        ctx.configure_mesh().draw().unwrap();

        ctx.draw_series($x_data.iter().zip($y_data.iter()).map(|(x, y)| {
            let x0 = SegmentValue::CenterOf(x);
            let x1 = SegmentValue::CenterOf(x);
            let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
            bar.set_margin(0, 0, 5, 5);
            bar
        })).unwrap();
    };
}