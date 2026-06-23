use genotype_test_generics_types::{
    Pair, Response, ResponseFailure, ResponsePair, ResponseString, ResponseSuccess,
};

fn main() {
    let _response_string_ok: ResponseString = Response::Success(ResponseSuccess {
        value: "Hello, world!".to_owned(),
    });

    let _response_string_failure: ResponseString = Response::Failure(ResponseFailure {
        error: "Something went wrong".to_owned(),
    });

    let _response_number_ok: Response<f64> = Response::Success(ResponseSuccess { value: 42.0 });

    let _response_pair_ok: ResponsePair = Response::Success(ResponseSuccess {
        value: Pair {
            left: "left value".to_owned(),
            right: 123.0,
        },
    });
}
