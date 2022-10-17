use aws_lambda_events::event::cloudwatch_logs::CloudwatchLogsEvent;use lambda_runtime::{run, service_fn, Error, LambdaEvent};


/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<CloudwatchLogsEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    println!("{:?}", event);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
