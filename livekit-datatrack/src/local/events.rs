// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    api::{DataTrackInfo, DataTrackOptions, LocalDataTrack, PublishError},
    packet::Handle,
};
use bytes::Bytes;
use std::sync::Arc;
use tokio::sync::oneshot;

// Manual `From` impls in place of `from_variants::FromVariants` so the crate
// graph doesn't drag in `darling 0.14` + `syn 1` (the laggard chain from the
// abandoned `from_variants` macro). Each `impl From<T> for Enum::Variant(T)`
// below corresponds to one tuple variant of the enum.

/// An external event handled by [`Manager`](super::manager::Manager).
#[derive(Debug)]
pub enum InputEvent {
    PublishRequest(PublishRequest),
    PublishCancelled(PublishCancelled),
    QueryPublished(QueryPublished),
    UnpublishRequest(UnpublishRequest),
    SfuPublishResponse(SfuPublishResponse),
    SfuUnpublishResponse(SfuUnpublishResponse),
    /// Republish all tracks.
    ///
    /// This must be sent after a full reconnect in order for existing publications
    /// to be recognized by the SFU. Each republished track will be assigned a new SID.
    ///
    RepublishTracks,
    /// Shutdown the manager and all associated tracks.
    Shutdown,
}

impl From<PublishRequest> for InputEvent {
    fn from(v: PublishRequest) -> Self { Self::PublishRequest(v) }
}
impl From<PublishCancelled> for InputEvent {
    fn from(v: PublishCancelled) -> Self { Self::PublishCancelled(v) }
}
impl From<QueryPublished> for InputEvent {
    fn from(v: QueryPublished) -> Self { Self::QueryPublished(v) }
}
impl From<UnpublishRequest> for InputEvent {
    fn from(v: UnpublishRequest) -> Self { Self::UnpublishRequest(v) }
}
impl From<SfuPublishResponse> for InputEvent {
    fn from(v: SfuPublishResponse) -> Self { Self::SfuPublishResponse(v) }
}
impl From<SfuUnpublishResponse> for InputEvent {
    fn from(v: SfuUnpublishResponse) -> Self { Self::SfuUnpublishResponse(v) }
}

/// An event produced by [`Manager`](super::manager::Manager) requiring external action.
#[derive(Debug)]
pub enum OutputEvent {
    SfuPublishRequest(SfuPublishRequest),
    SfuUnpublishRequest(SfuUnpublishRequest),
    /// Serialized packets are ready to be sent over the transport.
    PacketsAvailable(Vec<Bytes>),
}

impl From<SfuPublishRequest> for OutputEvent {
    fn from(v: SfuPublishRequest) -> Self { Self::SfuPublishRequest(v) }
}
impl From<SfuUnpublishRequest> for OutputEvent {
    fn from(v: SfuUnpublishRequest) -> Self { Self::SfuUnpublishRequest(v) }
}
impl From<Vec<Bytes>> for OutputEvent {
    fn from(v: Vec<Bytes>) -> Self { Self::PacketsAvailable(v) }
}

// MARK: - Input events

/// Client requested to publish a track.
///
/// Send using [`ManagerInput::publish_track`] and await the result.
///
/// [`ManagerInput::publish_track`]: super::manager::ManagerInput::publish_track
///
#[derive(Debug)]
pub struct PublishRequest {
    /// Publish options.
    pub(super) options: DataTrackOptions,
    /// Async completion channel.
    pub(super) result_tx: oneshot::Sender<Result<LocalDataTrack, PublishError>>,
}

/// Client request to publish a track has been cancelled (internal).
#[derive(Debug)]
pub struct PublishCancelled {
    /// Publisher handle of the pending publication.
    pub(super) handle: Handle,
}

/// Client request to unpublish a track (internal).
#[derive(Debug)]
pub struct UnpublishRequest {
    /// Publisher handle of the track to unpublish.
    pub(super) handle: Handle,
}

/// Get information about all currently published tracks.
///
/// Send using [`ManagerInput::query_tracks`] and await the result. This is used
/// to support sync state.
///
/// [`ManagerInput::query_tracks`]: super::manager::ManagerInput::query_tracks
///
#[derive(Debug)]
pub struct QueryPublished {
    pub(super) result_tx: oneshot::Sender<Vec<Arc<DataTrackInfo>>>,
}

/// SFU responded to a request to publish a data track.
///
/// Protocol equivalent: [`livekit_protocol::PublishDataTrackResponse`].
///
#[derive(Debug)]
pub struct SfuPublishResponse {
    /// Publisher handle of the track.
    pub handle: Handle,
    /// Outcome of the publish request.
    pub result: Result<DataTrackInfo, PublishError>,
}

/// SFU notification that a track has been unpublished.
///
/// Protocol equivalent: [`livekit_protocol::UnpublishDataTrackResponse`].
///
#[derive(Debug)]
pub struct SfuUnpublishResponse {
    /// Publisher handle of the track that was unpublished.
    pub handle: Handle,
}

// MARK: - Output events

/// Request sent to the SFU to publish a track.
///
/// Protocol equivalent: [`livekit_protocol::PublishDataTrackRequest`].
///
#[derive(Debug)]
pub struct SfuPublishRequest {
    pub handle: Handle,
    pub name: String,
    pub uses_e2ee: bool,
}

/// Request sent to the SFU to unpublish a track.
///
/// Protocol equivalent: [`livekit_protocol::UnpublishDataTrackRequest`].
///
#[derive(Debug)]
pub struct SfuUnpublishRequest {
    /// Publisher handle of the track to unpublish.
    pub handle: Handle,
}
