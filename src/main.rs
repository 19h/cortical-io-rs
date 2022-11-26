use cortical_io::{Cortical, TextSliceRequest};

#[tokio::main]
async fn main() {
    let foo = Cortical::new();

    let text = r#"Internally, the input documents will be stripped of all formatting in order to extract the single terms that are the basic unit of operation. The /text/tokenize endpoint is a way for the user to check what is actually retrieved in this process. A list of strings will be returned – each representing a sentence of the input text. Each sentence is simply a comma-separated list of terms that were found in the sentence. If you specify valid POS tags in the POStags field, only terms of these POS types will be returned. Using tokenization the user can thus retrieve the single terms of a text for example in order to build custom fingerprints for texts by using expressions."#;

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

    dbg!(
        foo.get_text_slices(
            text,
            Some(TextSliceRequest::new().with_get_fingerprint(false)),
        ).await
    );
}
