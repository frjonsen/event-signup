import axios from "axios";
import * as Sentry from "@sentry/react";
import { Dispatch, SetStateAction } from "react";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const reportRequestError = (error: any) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    let extra: any | undefined;
    if (axios.isAxiosError(error)) {
        extra = error.response?.data;
    }
    Sentry.captureException(error, { extra });
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const handleRequestError = (error: any, errorCodeSetter: Dispatch<SetStateAction<string | null>>) => {
    reportRequestError(error);
    if (errorCodeSetter !== undefined) {
    const backendErrorCode = error.response?.data?.errorCode;
        errorCodeSetter(backendErrorCode ?? "UNKNOWN_ERROR");
    }
}