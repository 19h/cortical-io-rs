use std::io::Write;

use cortical_io::{Cortical, TextSlice, TextSliceRequest};
use cortical_io::density::Density;
use cortical_io::image::{generate_height_image_from_vec, generate_image_from_fingerprint};
use cortical_io::similarity::FingerprintSimilarity;

#[cfg(feature = "client")]
#[tokio::main]
async fn main() {
    let cortical = Cortical::new();

    dbg!(cortical.get_terms(
        Some("en_general"),
        None,
        Some(true),
        None,
        Some(1000),
    ).await.unwrap());

    return;

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
    )
        .unwrap()
        .save("refvec.png")
        .unwrap();

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
    ).unwrap().save("kde.png").unwrap();

    let foo = Cortical::new();

    //let text = r#"Mercedes-Benz is to offer an online subscription service in the US to make its electric cars speed up quicker. For an annual cost of $1,200 (£991) excluding tax, the company will enable some of its vehicles to accelerate from 0-60mph a second faster. It comes after rival manufacturer BMW offered a subscription feature earlier this year - for heated seats. Mercedes has confirmed to BBC News it currently does not plan to introduce "Acceleration Increase" in the UK. It will be available for purchase in the US on the Mercedes-EQ EQE 350 and EQS 450 vehicles, as well as their SUV counterparts. According to the Mercedes US online store, the feature "electronically increases" the output of the car's motor, as well as the torque. All told, it estimates this amounts to a 20-24% increase in output, allowing a Mercedes-EQ 350 SUV to accelerate from 0-60mph in about 5.2 seconds, as opposed to 6.2 seconds without the subscription. Jack McKeown, Association of Scottish Motoring Writers president and motoring editor of the Courier newspaper, in Dundee, said Mercedes's new feature was "unsurprising but dispiriting". "When you pay a monthly subscription for a phone or for broadband, you're paying for the company to supply and maintain a data network," he said. "Mercedes is asking you to pay for hardware it has already installed in the car - and which it presumably already made a profit margin on when you bought the car. "Trying to leverage even more profit out of subscription services is a worrying trend and I hope there is a consumer backlash against it." In July, BMW faced a backlash when it announced customers could pay £25 per month to unlock heated seats and steering wheels in their cars. And in December 2021, Toyota announced it would charge some drivers $8 per month to remotely start their cars using a key fob. In 2019, Tesla introduced "Acceleration Boost", which makes its Model 3 vehicles accelerate from 0-60mph half a second faster for a one-time fee of $2,000. The Acceleration Increase subscription is listed as "coming soon" on the US Mercedes storefront, with no exact date given for its release."#;
    let text1 =
        r#"Missing cryptoqueen: Is Dr Ruja Ignatova the biggest Bitcoin holder?
With cryptomarkets in turmoil, files seen by the BBC suggest an unlikely Bitcoin investor have also lost a fortune - "the missing cryptoqueen", Dr Ruja Ignatova.
The scammer disappeared in as her cryptocurrency OneCoin was at its height - attracting billions from investors. Fraud and money-laundering charges in the US have led to her becoming one of the FBI's most wanted fugitives.
The Oxford-educated entrepreneur told investors she had created the "Bitcoin killer", but the files suggest she secretly amassed billions in her rival currency before she disappeared.
Details first surfaced in in leaked documents from Dubai's courts, posted online by a lawyer who crowned Dr Ruja - as she's known - the "most successful criminal in history".
A new book and upcoming episodes of BBC podcast The Missing Cryptoqueen investigate how Dr Ruja has stayed hidden - and whether this alleged crypto-hoard might help explain it.
The Dubai files
We have been able to independently verify some - but not all - of the information in the Dubai files.
At the very least, the leak suggests Dubai was an important financial route for Dr Ruja, something the FBI alluded to in naming the United Arab Emirates as one of five countries she has known connections to.
"You've got hundreds of millions of dollars at stake here," said Dr Jonathan Levy, the lawyer who first posted the files online, and who is relying on information from them in a compensation claim for OneCoin victims.
The claim was lodged with the Supreme Court of the British Indian Ocean Territory, chosen because it hosted a web domain allegedly used by OneCoin.
Dr Levy received thousands of documents, mostly in Arabic, from a whistleblower who he said thought it was wrong that people were being "unjustly enriched" in Dubai.
The 'Bitcoin deal'
The most tantalising claim made in Dr Levy's legal case is that a massive Bitcoin deal was struck with an Emirati royal, Sheikh Saoud bin Faisal Al Qassimi, the son of a wealthy business tycoon.
Story continues
The files further suggest that in Sheikh Saoud gave Dr Ruja four USB memory sticks containing bitcoin - worth €48.5m at the time.
In return, Dr Ruja handed over three cheques to Sheikh Saoud from Mashreq Bank, totalling around 210m Emirati dirhams, roughly €50m.
Prior to the alleged deal, Dubai's Mashreq Bank had begun closing Dr Ruja's accounts amid money-laundering concerns, so the cheques were unable to be cashed.
In the Dubai authorities unfroze Dr Ruja's funds, despite the fact that more than a year earlier the US Department of Justice had published an indictment for her, labelling OneCoin a "fraudulent cryptocurrency".
Months before the decision, Dr Ruja's former fund manager Mark Scott was also found guilty in New York of laundering $400m in OneCoin proceeds.
Asked by the BBC about the decision, Dubai's Public Prosecutor did not respond.
According to Dubai Court of Appeal records, on Sheikh Saoud was seeking to have Dr Ruja's funds handed to him - suggesting a deal of some kind did originally take place between the pair. Dr Ruja herself is named as a respondent, despite not being seen in public for nearly five years.
Dr Ruja and the sheikh
Little is known about Sheikh Saoud, but sources tell us the stocky body-building enthusiast is rarely seen in public.
He features in a YouTube video for an organisation called the Intergovernmental Collaborative Action Fund for Excellence (ICAFE) which claims to support education initiatives, but after the Dubai files were released, references to Sheikh Al Qassimi as ICAFE's "Secretary-General" disappeared from its website.
A recently launched cryptocurrency also lists the sheikh as its chairman. The Dubai files appear to show a once close relationship between the Al Qassimi family and Dr Ruja.
On Dubai's Mashreq Bank wrote to Dr Ruja to explain that it would be closing her personal accounts.
Eleven days later, an email admitted to a US court shows Dr Ruja writing to a OneCoin colleague about moving €50m out of Mashreq Bank. She mentions a meeting the following week with "one of the Sheiks [sic] in Dubai" where she would "try to get something done for us".
It is not known who Dr Ruja intended to meet or even if a meeting took place, but the files hint at one possible explanation.
A photo from the files seemingly dated but taken at an unknown place, pictures Dr Ruja standing next to Sheikh Faisal, the father of Sheikh Saoud.
The Al Qassimi family rules Sharjah, which borders Dubai and Ras Al Khaimah (RAK), the northernmost emirate. Sheikh Faisal did not respond to us about his family's relationship with Dr Ruja.
The files also include a diplomatic ID, issued to Ruja as a "special adviser" to ICAFE - the organisation with which Sheikh Saoud once held a senior role.
The organisation appears to be connected to the United Nations, but a spokesman for the UN Secretary-General could not find any records of it being affiliated through normal routes.
Co-founder Shariar Rahimi said ICAFE was "registered" with the UN, but failed to provide evidence of this. On Dr Ruja, he said any ICAFE documents provided to her came from Sheikh Saoud.
Some time after Dr Ruja's alleged Bitcoin deal, their relationship appears to have soured - one letter among the leaked files shows Sheikh Saoud dismissing Dr Ruja from a role as an ICAFE ambassador, before later becoming involved in a legal dispute over her assets.
The case between the pair concluded in Dubai's Court of Appeal on
Asked about the alleged Bitcoin deal, his relationship with Dr Ruja, and his role at ICAFE, Sheikh Saoud's lawyer did not directly respond, but wrote: "All the information you have is baseless".
The alleged Bitcoin transaction is said to have taken place using what's called cold-storage wallets, making it very difficult to ascertain if it actually happened.
Bitcoin transactions can often be traced because all transfers of the virtual currency between wallets are recorded on a publicly viewable database. However, the court documents do not include any details about which - or how many - wallets these bitcoins were stored on.
If she does still have them, Dr Ruja might find it difficult to move such a large amount of bitcoin.
Crypto-author David Birch thinks Bitcoin's reputation as an "anonymous" currency is inaccurate because law enforcement agencies are increasingly using clever algorithms to track coins as they flow through the system.
"Getting rid of a few billion dollars' worth is much harder than you think," he said.
If Dr Ruja still has the bitcoins, she would be one of the currency's largest holders. In her stake would have peaked at nearly $15bn, but at the time of writing it has dropped to around $5bn, still more than enough to help her stay hidden.
Follow Jamie Bartlett and Rob Byrne on Twitter
Catch up on The Missing Cryptoqueen podcast on BBC Sounds - the search for Dr Ruja Ignatova continues in"#;

    let slices1 =
        foo.get_text_slices(
            text1,
            Some(TextSliceRequest::new().with_get_fingerprint(true)),
        ).await.unwrap();

    for slice in slices1.iter() {
        println!("{}\n", slice.text);
    }

    return;

    slices1.iter()
        .enumerate()
        .for_each(|(i, slice)| {
            println!("-- slice {} --", i);
            println!("{}", slice.text);

            let mut density =
                Density::new(
                    slice
                        .fingerprint
                        .as_ref()
                        .unwrap()
                        .expand(16384)
                        .iter()
                        .map(|v| *v as u32)
                        .collect::<Vec<_>>()
                        .as_slice(),
                );

            density.filter_points_min(30);

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

            let img =
                generate_image_from_fingerprint(
                    slice.fingerprint.as_ref().unwrap(),
                    10,
                );

            img.unwrap().save(format!("slice-{}.png", i)).unwrap();
        });

    //let _left = slices1[0].fingerprint.as_ref().unwrap();
    //let _right = slices2[0].fingerprint.as_ref().unwrap();

    //let mut similarities = Vec::new();

    // for i in 0..slices.len() {
    //     for j in 0..slices.len() {
    //         if i == j {
    //             continue;
    //         }
    //
    //         let similarity = FingerprintSimilarity::new(
    //             slices[i].fingerprint.as_ref().unwrap(),
    //             slices[j].fingerprint.as_ref().unwrap(),
    //         ).weighted_scoring();
    //
    //         similarities.push((similarity, &slices[i].text, &slices[j].text));
    //     }
    // }
    //
    // similarities
    //     .dedup_by(|a, b| {
    //         (a.1.eq(b.1) && a.2.eq(b.2))
    //             || (a.1.eq(b.2) && a.2.eq(b.1))
    //     });
    //
    // similarities
    //     .sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    //
    // for similarity in similarities.iter() {
    //     println!(
    //         "{}:\n\t- {}\n\t- {}",
    //         similarity.0,
    //         &similarity.1[0..similarity.1.len().min(100)],
    //         &similarity.2[0..similarity.2.len().min(100)],
    //     );
    // }
    //
    // slices.iter()
    //     .enumerate()
    //     .for_each(|(i, slice)| {
    //         let img = generate_image_from_fingerprint(
    //             slice.fingerprint.as_ref().unwrap(),
    //             10,
    //         );
    //
    //         img.unwrap().save(format!("slice-{}.png", i)).unwrap();
    //     });
}

#[cfg(not(feature = "client"))]
fn main() {
    println!("This example requires the client feature to be enabled.");
}