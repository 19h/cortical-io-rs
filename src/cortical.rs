use std::io::Write;
use cortical_io::{Cortical, TextSliceRequest};
use cortical_io::density::Density;
use cortical_io::image::{generate_height_image_from_vec, generate_image_from_fingerprint};
use cortical_io::similarity::FingerprintSimilarity;

#[cfg(feature = "client")]
#[tokio::main]
async fn main() {
    let file =
        std::fs::read_to_string("./refvec.txt")
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

    let mut density =
        Density::new(&file);

    density.filter_points_min(30);

    generate_height_image_from_vec(
        density.get_data(),
        10,
        |p, _|
            if p == 0 {
                [255, 255, 255]
            } else {
                [255 - p, 255 - p, 255]
            },
    ).save("refvec.png").unwrap();

    let kde = density.kde();

    let densest_points = &kde.densest_points;
    let kde_vec = kde.get_kde_data();

    // create a string including 20 values per line, separates by space
    let kde_str =
        kde_vec
            .iter()
            .enumerate()
            .fold(
                String::new(),
                |mut acc, (i, &x)| {
                    let x = x / 50;

                    if i % 128 == 0 {
                        acc.push_str(&format!("\n"));
                    } else {
                        acc.push_str(
                            &format!(
                                "{} ",
                                if x < 10 {
                                    format!("  {}", x)
                                } else if x < 100 {
                                    format!(" {}", x)
                                } else {
                                    format!("{}", x)
                                }
                            )
                        );
                    }

                    acc
                },
            );

    let file = std::fs::File::create("kde.txt").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(kde_str.as_bytes()).unwrap();

    generate_height_image_from_vec(
        kde_vec.as_slice(),
        10,
        |p, i|
            match p {
                0 => [255, 255, 255],
                _ if densest_points.contains(&i) => [255, 0, 0],
                _ => [255 - p, 255 - p, 255],
            },
    ).save("kde.png").unwrap();


    return;

    let foo = Cortical::new();

    let text = r#"Mercedes-Benz is to offer an online subscription service in the US to make its electric cars speed up quicker. For an annual cost of $1,200 (£991) excluding tax, the company will enable some of its vehicles to accelerate from 0-60mph a second faster. It comes after rival manufacturer BMW offered a subscription feature earlier this year - for heated seats. Mercedes has confirmed to BBC News it currently does not plan to introduce "Acceleration Increase" in the UK. It will be available for purchase in the US on the Mercedes-EQ EQE 350 and EQS 450 vehicles, as well as their SUV counterparts. According to the Mercedes US online store, the feature "electronically increases" the output of the car's motor, as well as the torque. All told, it estimates this amounts to a 20-24% increase in output, allowing a Mercedes-EQ 350 SUV to accelerate from 0-60mph in about 5.2 seconds, as opposed to 6.2 seconds without the subscription. Jack McKeown, Association of Scottish Motoring Writers president and motoring editor of the Courier newspaper, in Dundee, said Mercedes's new feature was "unsurprising but dispiriting". "When you pay a monthly subscription for a phone or for broadband, you're paying for the company to supply and maintain a data network," he said. "Mercedes is asking you to pay for hardware it has already installed in the car - and which it presumably already made a profit margin on when you bought the car. "Trying to leverage even more profit out of subscription services is a worrying trend and I hope there is a consumer backlash against it." In July, BMW faced a backlash when it announced customers could pay £25 per month to unlock heated seats and steering wheels in their cars. And in December 2021, Toyota announced it would charge some drivers $8 per month to remotely start their cars using a key fob. In 2019, Tesla introduced "Acceleration Boost", which makes its Model 3 vehicles accelerate from 0-60mph half a second faster for a one-time fee of $2,000. The Acceleration Increase subscription is listed as "coming soon" on the US Mercedes storefront, with no exact date given for its release."#;

    let slices =
        foo.get_text_slices(
            text,
            Some(TextSliceRequest::new().with_get_fingerprint(true)),
        ).await.unwrap();

    let _left = slices[0].fingerprint.as_ref().unwrap();
    let _right = slices[1].fingerprint.as_ref().unwrap();

    let mut similarities = Vec::new();

    for i in 0..slices.len() {
        for j in 0..slices.len() {
            if i == j {
                continue;
            }

            let similarity = FingerprintSimilarity::new(
                slices[i].fingerprint.as_ref().unwrap(),
                slices[j].fingerprint.as_ref().unwrap(),
            ).weighted_scoring();

            similarities.push((similarity, &slices[i].text, &slices[j].text));
        }
    }

    similarities
        .dedup_by(|a, b| {
            (a.1.eq(b.1) && a.2.eq(b.2))
                || (a.1.eq(b.2) && a.2.eq(b.1))
        });

    similarities
        .sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    for similarity in similarities.iter() {
        println!(
            "{}:\n\t- {}\n\t- {}",
            similarity.0,
            &similarity.1[0..similarity.1.len().min(100)],
            &similarity.2[0..similarity.2.len().min(100)],
        );
    }

    slices.iter()
        .enumerate()
        .for_each(|(i, slice)| {
            let img = generate_image_from_fingerprint(
                slice.fingerprint.as_ref().unwrap(),
                10,
            );

            img.save(format!("slice-{}.png", i)).unwrap();
        });
}

#[cfg(not(feature = "client"))]
fn main() {
    println!("This example requires the client feature to be enabled.");
}