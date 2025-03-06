import * as agw from "aws-cdk-lib/aws-apigatewayv2";
import { LogGroup, RetentionDays } from "aws-cdk-lib/aws-logs";
import { Construct } from "constructs";
import * as iam from "aws-cdk-lib/aws-iam";

export class HttpApi extends agw.HttpApi {
  constructor(scope: Construct) {
    super(scope, "Gateway", {});

    const accessLog = new LogGroup(this, "AccessLog", {
      retention: RetentionDays.ONE_DAY,
    });

    const stage = this.defaultStage!.node.defaultChild as agw.CfnStage;
    stage.accessLogSettings = {
      destinationArn: accessLog.logGroupArn,
      format:
        '$context.identity.sourceIp - - [$context.requestTime] "$context.httpMethod $context.routeKey $context.protocol" $context.status $context.responseLength $context.requestId',
    };

    // const role = new iam.Role(this, "AccessLogRole", {
    //   assumedBy: new iam.ServicePrincipal("apigateway.amazonaws.com"),
    // });

    // role.addToPrincipalPolicy(
    //   new iam.PolicyStatement({
    //     actions: [
    //       "logs:CreateLogGroup",
    //       "logs:CreateLogStream",
    //       "logs:DescribeLogGroups",
    //       "logs:DescribeLogStreams",
    //       "logs:PutLogEvents",
    //       "logs:GetLogEvents",
    //       "logs:FilterLogEvents",
    //     ],
    //     resources: ["*"],
    //   }),
    // );

    // accessLog.grantWrite(role);
  }
}
