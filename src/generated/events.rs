use crate::generated::messages::webcast::*;
use crate::generated::messages::webcast::webcast_response::Message;
///
/// Generated file
///
pub struct TikTokSocialEvent {
    pub raw_data: WebcastSocialMessage,
}
pub struct TikTokPollEvent {
    pub raw_data: WebcastPollMessage,
}
pub struct TikTokMsgDetectEvent {
    pub raw_data: WebcastMsgDetectMessage,
}
pub struct TikTokRankTextEvent {
    pub raw_data: WebcastRankTextMessage,
}
pub struct TikTokSubNotifyEvent {
    pub raw_data: WebcastSubNotifyMessage,
}
pub struct TikTokChatEvent {
    pub raw_data: WebcastChatMessage,
}
pub struct TikTokCaptionEvent {
    pub raw_data: WebcastCaptionMessage,
}
pub struct TikTokEnvelopeEvent {
    pub raw_data: WebcastEnvelopeMessage,
}
pub struct TikTokControlEvent {
    pub raw_data: WebcastControlMessage,
}
pub struct TikTokLinkMicFanTicketMethodEvent {
    pub raw_data: WebcastLinkMicFanTicketMethod,
}
pub struct TikTokOecLiveShoppingEvent {
    pub raw_data: WebcastOecLiveShoppingMessage,
}
pub struct TikTokRoomVerifyEvent {
    pub raw_data: RoomVerifyMessage,
}
pub struct TikTokEmoteChatEvent {
    pub raw_data: WebcastEmoteChatMessage,
}
pub struct TikTokLiveIntroEvent {
    pub raw_data: WebcastLiveIntroMessage,
}
pub struct TikTokInRoomBannerEvent {
    pub raw_data: WebcastInRoomBannerMessage,
}
pub struct TikTokLinkLayerEvent {
    pub raw_data: WebcastLinkLayerMessage,
}
pub struct TikTokHourlyRankEvent {
    pub raw_data: WebcastHourlyRankMessage,
}
pub struct TikTokRoomEvent {
    pub raw_data: WebcastRoomMessage,
}
pub struct TikTokLinkEvent {
    pub raw_data: WebcastLinkMessage,
}
pub struct TikTokQuestionNewEvent {
    pub raw_data: WebcastQuestionNewMessage,
}
pub struct TikTokResponseEvent {
    pub raw_data: WebcastResponse,
}
pub struct TikTokMemberEvent {
    pub raw_data: WebcastMemberMessage,
}
pub struct TikTokLinkMicArmiesEvent {
    pub raw_data: WebcastLinkMicArmies,
}
pub struct TikTokRoomUserSeqEvent {
    pub raw_data: WebcastRoomUserSeqMessage,
}
pub struct TikTokPushFrameEvent {
    pub raw_data: WebcastPushFrame,
}
pub struct TikTokImDeleteEvent {
    pub raw_data: WebcastImDeleteMessage,
}
pub struct TikTokLinkmicBattleTaskEvent {
    pub raw_data: WebcastLinkmicBattleTaskMessage,
}
pub struct TikTokLikeEvent {
    pub raw_data: WebcastLikeMessage,
}
pub struct TikTokLinkMicBattleEvent {
    pub raw_data: WebcastLinkMicBattle,
}
pub struct TikTokGiftEvent {
    pub raw_data: WebcastGiftMessage,
}
pub struct TikTokUnauthorizedMemberEvent {
    pub raw_data: WebcastUnauthorizedMemberMessage,
}
pub struct TikTokRankUpdateEvent {
    pub raw_data: WebcastRankUpdateMessage,
}
pub struct TikTokGoalUpdateEvent {
    pub raw_data: WebcastGoalUpdateMessage,
}
pub struct TikTokLinkMicBattlePunishFinishEvent {
    pub raw_data: WebcastLinkMicBattlePunishFinish,
}
pub struct TikTokSystemEvent {
    pub raw_data: WebcastSystemMessage,
}
pub struct TikTokBarrageEvent {
    pub raw_data: WebcastBarrageMessage,
}
pub struct TikTokRoomPinEvent {
    pub raw_data: WebcastRoomPinMessage,
}
pub struct TikTokLinkMicMethodEvent {
    pub raw_data: WebcastLinkMicMethod,
}
pub enum TikTokLiveEvent {
    OnInRoomBanner(TikTokInRoomBannerEvent),
    OnQuestionNew(TikTokQuestionNewEvent),
    OnSocial(TikTokSocialEvent),
    OnLinkMicBattlePunishFinish(TikTokLinkMicBattlePunishFinishEvent),
    OnLinkmicBattleTask(TikTokLinkmicBattleTaskEvent),
    OnMsgDetect(TikTokMsgDetectEvent),
    OnOecLiveShopping(TikTokOecLiveShoppingEvent),
    OnRoomVerify(TikTokRoomVerifyEvent),
    OnCaption(TikTokCaptionEvent),
    OnLinkLayer(TikTokLinkLayerEvent),
    OnMember(TikTokMemberEvent),
    OnGoalUpdate(TikTokGoalUpdateEvent),
    OnLink(TikTokLinkEvent),
    OnRankUpdate(TikTokRankUpdateEvent),
    OnUnauthorizedMember(TikTokUnauthorizedMemberEvent),
    OnLinkMicMethod(TikTokLinkMicMethodEvent),
    OnLike(TikTokLikeEvent),
    OnRoomPin(TikTokRoomPinEvent),
    OnResponse(TikTokResponseEvent),
    OnRoom(TikTokRoomEvent),
    OnRoomUserSeq(TikTokRoomUserSeqEvent),
    OnGift(TikTokGiftEvent),
    OnLinkMicArmies(TikTokLinkMicArmiesEvent),
    OnPushFrame(TikTokPushFrameEvent),
    OnBarrage(TikTokBarrageEvent),
    OnEmoteChat(TikTokEmoteChatEvent),
    OnSystem(TikTokSystemEvent),
    OnEnvelope(TikTokEnvelopeEvent),
    OnChat(TikTokChatEvent),
    OnSubNotify(TikTokSubNotifyEvent),
    OnPoll(TikTokPollEvent),
    OnControl(TikTokControlEvent),
    OnLinkMicBattle(TikTokLinkMicBattleEvent),
    OnImDelete(TikTokImDeleteEvent),
    OnLiveIntro(TikTokLiveIntroEvent),
    OnRankText(TikTokRankTextEvent),
    OnHourlyRank(TikTokHourlyRankEvent),
    OnLinkMicFanTicketMethod(TikTokLinkMicFanTicketMethodEvent),
}
