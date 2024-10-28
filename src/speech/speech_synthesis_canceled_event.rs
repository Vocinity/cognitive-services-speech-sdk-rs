use crate::common::{CancellationErrorCode, CancellationReason, PropertyId};
use crate::error::{convert_err, Result};
use crate::ffi::{
    synth_result_get_canceled_error_code, synth_result_get_reason_canceled, SPXEVENTHANDLE,
};
use crate::speech::SpeechSynthesisEvent;
use log::*;

/// Synthesis event extending *SpeechSynthesisEvent* passed into callback *set_canceled_cb*.
#[derive(Debug)]
pub struct SpeechSynthesisCanceledEvent {
    pub base: SpeechSynthesisEvent,
    pub reason: CancellationReason,
    pub error_code: CancellationErrorCode,
    pub error_details: String,
}

impl SpeechSynthesisCanceledEvent {
    pub fn from_handle(handle: SPXEVENTHANDLE) -> Result<SpeechSynthesisCanceledEvent> {
        unsafe {
            let base = SpeechSynthesisEvent::from_handle(handle)?;
            let mut reason = 0;
            let ret = synth_result_get_reason_canceled(base.result.handle.inner(), &mut reason);
            convert_err(
                ret,
                "SpeechSynthesisCanceledEvent::from_handle(synth_result_get_reason_canceled) error",
            )?;

            let mut error_code = 0;
            let ret =
                synth_result_get_canceled_error_code(base.result.handle.inner(), &mut error_code);
            convert_err(
                ret,
                "SpeechSynthesisCanceledEvent::from_handle(synth_result_get_canceled_error_code) error",
            )?;

            let error_details;
            let error_details_res = base.result.properties.get_property(
                PropertyId::CancellationDetailsReasonDetailedText,
                "".to_string(),
            );
            if let Err(err) = error_details_res {
                warn!(
                    "Error when getting SpeechServiceResponseJsonErrorDetails {:?}",
                    err
                );
                error_details = "".to_owned();
            } else {
                error_details = error_details_res.unwrap();
            }

            Ok(SpeechSynthesisCanceledEvent {
                base,
                reason: CancellationReason::from_u32(reason),
                error_code: CancellationErrorCode::from_u32(error_code),
                error_details,
            })
        }
    }
}
