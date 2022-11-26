use cortical_io::{Cortical, TextSliceRequest};
use cortical_io::image::generate_image_from_fingerprint;
use cortical_io::similarity::FingerprintSimilarity;

#[tokio::main]
async fn main() {
    let foo = Cortical::new();

    let text = r#"Mercedes-Benz is to offer an online subscription service in the US to make its electric cars speed up quicker. For an annual cost of $1,200 (£991) excluding tax, the company will enable some of its vehicles to accelerate from 0-60mph a second faster. It comes after rival manufacturer BMW offered a subscription feature earlier this year - for heated seats. Mercedes has confirmed to BBC News it currently does not plan to introduce "Acceleration Increase" in the UK. It will be available for purchase in the US on the Mercedes-EQ EQE 350 and EQS 450 vehicles, as well as their SUV counterparts. According to the Mercedes US online store, the feature "electronically increases" the output of the car's motor, as well as the torque. All told, it estimates this amounts to a 20-24% increase in output, allowing a Mercedes-EQ 350 SUV to accelerate from 0-60mph in about 5.2 seconds, as opposed to 6.2 seconds without the subscription. Jack McKeown, Association of Scottish Motoring Writers president and motoring editor of the Courier newspaper, in Dundee, said Mercedes's new feature was "unsurprising but dispiriting". "When you pay a monthly subscription for a phone or for broadband, you're paying for the company to supply and maintain a data network," he said. "Mercedes is asking you to pay for hardware it has already installed in the car - and which it presumably already made a profit margin on when you bought the car. "Trying to leverage even more profit out of subscription services is a worrying trend and I hope there is a consumer backlash against it." In July, BMW faced a backlash when it announced customers could pay £25 per month to unlock heated seats and steering wheels in their cars. And in December 2021, Toyota announced it would charge some drivers $8 per month to remotely start their cars using a key fob. In 2019, Tesla introduced "Acceleration Boost", which makes its Model 3 vehicles accelerate from 0-60mph half a second faster for a one-time fee of $2,000. The Acceleration Increase subscription is listed as "coming soon" on the US Mercedes storefront, with no exact date given for its release."#;

    //foo.get_compare(
    //    (
    //        text,
    //        "foo"
    //    ),
    //    None,
    //).await.unwrap();

    //foo.get_retinas().await.unwrap();

    //foo.get_text_analysis(
    //    text,
    //    None,
    //).await;

    let mut slices =
        foo.get_text_slices(
            text,
            Some(TextSliceRequest::new().with_get_fingerprint(true)),
        ).await.unwrap();

    let left = slices[0].fingerprint.as_ref().unwrap();
    let right = slices[1].fingerprint.as_ref().unwrap();

    let mut similarities = Vec::new();

    for i in 0..slices.len() {
        for j in 0..slices.len() {
            if i == j {
                continue;
            }

            let similarity = FingerprintSimilarity::new(
                &slices[i].fingerprint.as_ref().unwrap(),
                &slices[j].fingerprint.as_ref().unwrap(),
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
