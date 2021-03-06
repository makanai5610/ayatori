resource "aws_sns_topic_subscription" "ServiceA_Event" {
  topic_arn = "ServiceA_Event"
  protocol  = "sqs"
  endpoint  = ServiceC
}

resource "aws_sns_topic_subscription" "ServiceB_Event" {
  topic_arn = "ServiceB_Event"
  protocol  = "sqs"
  endpoint  = ServiceC
}

resource "aws_sns_topic_subscription" "ServiceD_Event" {
  topic_arn = "ServiceD_Event"
  protocol  = "sqs"
  endpoint  = ServiceC
}
