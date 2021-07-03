// autogenerated file
#![allow(unused_imports)]

use crate::types::*;
use packet_derive::{ClientBoundPacket, ServerBoundPacket};
use async_std::io::Cursor;
use std::fmt::{Display, Formatter};

pub mod client {
	use super::*;

/* TODO incomplete struct SpawnEntity
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x00]
	pub struct SpawnEntity {
		pub entity_id: VarIntField,
		// TODO pub object_uuid: Uuid,
		pub r#type: ByteField,
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub pitch: ByteField,
		pub yaw: ByteField,
		pub object_data: IntField,
		pub velocity_x: ShortField,
		pub velocity_y: ShortField,
		pub velocity_z: ShortField,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x01]
	pub struct SpawnEntityExperienceOrb {
		pub entity_id: VarIntField,
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub count: ShortField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x02]
	pub struct SpawnEntityWeather {
		pub entity_id: VarIntField,
		pub r#type: ByteField,
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
	}


/* TODO incomplete struct SpawnEntityLiving
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x03]
	pub struct SpawnEntityLiving {
		pub entity_id: VarIntField,
		// TODO pub entity_uuid: Uuid,
		pub r#type: VarIntField,
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub yaw: ByteField,
		pub pitch: ByteField,
		pub head_pitch: ByteField,
		pub velocity_x: ShortField,
		pub velocity_y: ShortField,
		pub velocity_z: ShortField,
		// TODO pub metadata: EntityMetadata,
	}
*/

/* TODO incomplete struct SpawnEntityPainting
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x04]
	pub struct SpawnEntityPainting {
		pub entity_id: VarIntField,
		// TODO pub entity_uuid: Uuid,
		pub title: StringField,
		pub location: PositionField<340>,
		pub direction: UByteField,
	}
*/

/* TODO incomplete struct NamedEntitySpawn
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x05]
	pub struct NamedEntitySpawn {
		pub entity_id: VarIntField,
		// TODO pub player_uuid: Uuid,
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub yaw: ByteField,
		pub pitch: ByteField,
		// TODO pub metadata: EntityMetadata,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x06]
	pub struct Animation {
		pub entity_id: VarIntField,
		pub animation: UByteField,
	}


/* TODO incomplete struct Statistics
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x07]
	pub struct Statistics {
		// TODO pub entries: Array { count_ty: Varint },
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x08]
	pub struct BlockBreakAnimation {
		pub entity_id: VarIntField,
		pub location: PositionField<340>,
		pub destroy_stage: ByteField,
	}


/* TODO incomplete struct TileEntityData
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x09]
	pub struct TileEntityData {
		pub location: PositionField<340>,
		pub action: UByteField,
		// TODO pub nbt_data: OptionalNbt,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x0a]
	pub struct BlockAction {
		pub location: PositionField<340>,
		pub byte_1: UByteField,
		pub byte_2: UByteField,
		pub block_id: VarIntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x0b]
	pub struct BlockChange {
		pub location: PositionField<340>,
		pub r#type: VarIntField,
	}


/* TODO incomplete struct BossBar
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x0c]
	pub struct BossBar {
		// TODO pub entity_uuid: Uuid,
		pub action: VarIntField,
		// TODO pub title: Switch,
		// TODO pub health: Switch,
		// TODO pub color: Switch,
		// TODO pub dividers: Switch,
		// TODO pub flags: Switch,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x0d]
	pub struct Difficulty {
		pub difficulty: UByteField,
	}


/* TODO incomplete struct TabComplete
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x0e]
	pub struct TabComplete {
		// TODO pub matches: Array { count_ty: Varint },
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x0f]
	pub struct Chat {
		pub message: StringField,
		pub position: ByteField,
	}


/* TODO incomplete struct MultiBlockChange
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x10]
	pub struct MultiBlockChange {
		pub chunk_x: IntField,
		pub chunk_z: IntField,
		// TODO pub records: Array { count_ty: Varint },
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x11]
	pub struct Transaction {
		pub window_id: ByteField,
		pub action: ShortField,
		pub accepted: BoolField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x12]
	pub struct CloseWindow {
		pub window_id: UByteField,
	}


/* TODO incomplete struct OpenWindow
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x13]
	pub struct OpenWindow {
		pub window_id: UByteField,
		pub inventory_type: StringField,
		pub window_title: StringField,
		pub slot_count: UByteField,
		// TODO pub entity_id: Switch,
	}
*/

/* TODO incomplete struct WindowItems
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x14]
	pub struct WindowItems {
		pub window_id: UByteField,
		// TODO pub items: Array { count_ty: I16 },
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x15]
	pub struct CraftProgressBar {
		pub window_id: UByteField,
		pub property: ShortField,
		pub value: ShortField,
	}


/* TODO incomplete struct SetSlot
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x16]
	pub struct SetSlot {
		pub window_id: ByteField,
		pub slot: ShortField,
		// TODO pub item: Slot,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x17]
	pub struct SetCooldown {
		pub item_id: VarIntField,
		pub cooldown_ticks: VarIntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x18]
	pub struct CustomPayload {
		pub channel: StringField,
		pub data: RestOfPacketByteArrayField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x19]
	pub struct NamedSoundEffect {
		pub sound_name: StringField,
		pub sound_category: VarIntField,
		pub x: IntField,
		pub y: IntField,
		pub z: IntField,
		pub volume: FloatField,
		pub pitch: FloatField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x1a]
	pub struct KickDisconnect {
		pub reason: StringField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x1b]
	pub struct EntityStatus {
		pub entity_id: IntField,
		pub entity_status: ByteField,
	}


/* TODO incomplete struct Explosion
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x1c]
	pub struct Explosion {
		pub x: FloatField,
		pub y: FloatField,
		pub z: FloatField,
		pub radius: FloatField,
		// TODO pub affected_block_offsets: Array { count_ty: I32 },
		pub player_motion_x: FloatField,
		pub player_motion_y: FloatField,
		pub player_motion_z: FloatField,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x1d]
	pub struct UnloadChunk {
		pub chunk_x: IntField,
		pub chunk_z: IntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x1e]
	pub struct GameStateChange {
		pub reason: UByteField,
		pub game_mode: FloatField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x1f]
	pub struct KeepAlive {
		pub keep_alive_id: LongField,
	}


/* TODO incomplete struct MapChunk
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x20]
	pub struct MapChunk {
		pub x: IntField,
		pub z: IntField,
		pub ground_up: BoolField,
		pub bit_map: VarIntField,
		pub chunk_data: VarIntThenByteArrayField,
		// TODO pub block_entities: Array { count_ty: Varint },
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x21]
	pub struct WorldEvent {
		pub effect_id: IntField,
		pub location: PositionField<340>,
		pub data: IntField,
		pub global: BoolField,
	}


/* TODO incomplete struct WorldParticles
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x22]
	pub struct WorldParticles {
		pub particle_id: IntField,
		pub long_distance: BoolField,
		pub x: FloatField,
		pub y: FloatField,
		pub z: FloatField,
		pub offset_x: FloatField,
		pub offset_y: FloatField,
		pub offset_z: FloatField,
		pub particle_data: FloatField,
		pub particles: IntField,
		// TODO pub data: Switch,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x23]
	pub struct Login {
		pub entity_id: IntField,
		pub game_mode: UByteField,
		pub dimension: IntField,
		pub difficulty: UByteField,
		pub max_players: UByteField,
		pub level_type: StringField,
		pub reduced_debug_info: BoolField,
	}


/* TODO incomplete struct Map
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x24]
	pub struct Map {
		pub item_damage: VarIntField,
		pub scale: ByteField,
		pub tracking_position: BoolField,
		// TODO pub icons: Array { count_ty: Varint },
		pub columns: ByteField,
		// TODO pub rows: Switch,
		// TODO pub x: Switch,
		// TODO pub y: Switch,
		// TODO pub data: Switch,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x25]
	pub struct Entity {
		pub entity_id: VarIntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x26]
	pub struct RelEntityMove {
		pub entity_id: VarIntField,
		pub d_x: ShortField,
		pub d_y: ShortField,
		pub d_z: ShortField,
		pub on_ground: BoolField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x27]
	pub struct EntityMoveLook {
		pub entity_id: VarIntField,
		pub d_x: ShortField,
		pub d_y: ShortField,
		pub d_z: ShortField,
		pub yaw: ByteField,
		pub pitch: ByteField,
		pub on_ground: BoolField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x28]
	pub struct EntityLook {
		pub entity_id: VarIntField,
		pub yaw: ByteField,
		pub pitch: ByteField,
		pub on_ground: BoolField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x29]
	pub struct VehicleMove {
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub yaw: FloatField,
		pub pitch: FloatField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x2a]
	pub struct OpenSignEntity {
		pub location: PositionField<340>,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x2b]
	pub struct CraftRecipeResponse {
		pub window_id: ByteField,
		pub recipe: VarIntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x2c]
	pub struct Abilities {
		pub flags: ByteField,
		pub flying_speed: FloatField,
		pub walking_speed: FloatField,
	}


/* TODO incomplete struct CombatEvent
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x2d]
	pub struct CombatEvent {
		pub event: VarIntField,
		// TODO pub duration: Switch,
		// TODO pub player_id: Switch,
		// TODO pub entity_id: Switch,
		// TODO pub message: Switch,
	}
*/

/* TODO incomplete struct PlayerInfo
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x2e]
	pub struct PlayerInfo {
		pub action: VarIntField,
		// TODO pub data: Array { count_ty: Varint },
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x2f]
	pub struct Position {
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub yaw: FloatField,
		pub pitch: FloatField,
		pub flags: ByteField,
		pub teleport_id: VarIntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x30]
	pub struct Bed {
		pub entity_id: VarIntField,
		pub location: PositionField<340>,
	}


/* TODO incomplete struct UnlockRecipes
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x31]
	pub struct UnlockRecipes {
		pub action: VarIntField,
		pub crafting_book_open: BoolField,
		pub filtering_craftable: BoolField,
		// TODO pub recipes_1: Array { count_ty: Varint },
		// TODO pub recipes_2: Switch,
	}
*/

/* TODO incomplete struct EntityDestroy
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x32]
	pub struct EntityDestroy {
		// TODO pub entity_ids: Array { count_ty: Varint },
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x33]
	pub struct RemoveEntityEffect {
		pub entity_id: VarIntField,
		pub effect_id: ByteField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x34]
	pub struct ResourcePackSend {
		pub url: StringField,
		pub hash: StringField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x35]
	pub struct Respawn {
		pub dimension: IntField,
		pub difficulty: UByteField,
		pub gamemode: UByteField,
		pub level_type: StringField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x36]
	pub struct EntityHeadRotation {
		pub entity_id: VarIntField,
		pub head_yaw: ByteField,
	}


/* TODO incomplete struct SelectAdvancementTab
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x37]
	pub struct SelectAdvancementTab {
		// TODO pub id: Option(String),
	}
*/

/* TODO incomplete struct WorldBorder
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x38]
	pub struct WorldBorder {
		pub action: VarIntField,
		// TODO pub radius: Switch,
		// TODO pub x: Switch,
		// TODO pub z: Switch,
		// TODO pub old_radius: Switch,
		// TODO pub new_radius: Switch,
		// TODO pub speed: Switch,
		// TODO pub portal_boundary: Switch,
		// TODO pub warning_time: Switch,
		// TODO pub warning_blocks: Switch,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x39]
	pub struct Camera {
		pub camera_id: VarIntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x3a]
	pub struct HeldItemSlot {
		pub slot: ByteField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x3b]
	pub struct ScoreboardDisplayObjective {
		pub position: ByteField,
		pub name: StringField,
	}


/* TODO incomplete struct EntityMetadata
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x3c]
	pub struct EntityMetadata {
		pub entity_id: VarIntField,
		// TODO pub metadata: EntityMetadata,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x3d]
	pub struct AttachEntity {
		pub entity_id: IntField,
		pub vehicle_id: IntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x3e]
	pub struct EntityVelocity {
		pub entity_id: VarIntField,
		pub velocity_x: ShortField,
		pub velocity_y: ShortField,
		pub velocity_z: ShortField,
	}


/* TODO incomplete struct EntityEquipment
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x3f]
	pub struct EntityEquipment {
		pub entity_id: VarIntField,
		pub slot: VarIntField,
		// TODO pub item: Slot,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x40]
	pub struct Experience {
		pub experience_bar: FloatField,
		pub level: VarIntField,
		pub total_experience: VarIntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x41]
	pub struct UpdateHealth {
		pub health: FloatField,
		pub food: VarIntField,
		pub food_saturation: FloatField,
	}


/* TODO incomplete struct ScoreboardObjective
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x42]
	pub struct ScoreboardObjective {
		pub name: StringField,
		pub action: ByteField,
		// TODO pub display_text: Switch,
		// TODO pub r#type: Switch,
	}
*/

/* TODO incomplete struct SetPassengers
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x43]
	pub struct SetPassengers {
		pub entity_id: VarIntField,
		// TODO pub passengers: Array { count_ty: Varint },
	}
*/

/* TODO incomplete struct Teams
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x44]
	pub struct Teams {
		pub team: StringField,
		pub mode: ByteField,
		// TODO pub name: Switch,
		// TODO pub prefix: Switch,
		// TODO pub suffix: Switch,
		// TODO pub friendly_fire: Switch,
		// TODO pub name_tag_visibility: Switch,
		// TODO pub collision_rule: Switch,
		// TODO pub color: Switch,
		// TODO pub players: Switch,
	}
*/

/* TODO incomplete struct ScoreboardScore
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x45]
	pub struct ScoreboardScore {
		pub item_name: StringField,
		pub action: ByteField,
		pub score_name: StringField,
		// TODO pub value: Switch,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x46]
	pub struct SpawnPosition {
		pub location: PositionField<340>,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x47]
	pub struct UpdateTime {
		pub age: LongField,
		pub time: LongField,
	}


/* TODO incomplete struct Title
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x48]
	pub struct Title {
		pub action: VarIntField,
		// TODO pub text: Switch,
		// TODO pub fade_in: Switch,
		// TODO pub stay: Switch,
		// TODO pub fade_out: Switch,
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x49]
	pub struct SoundEffect {
		pub sound_id: VarIntField,
		pub sound_category: VarIntField,
		pub x: IntField,
		pub y: IntField,
		pub z: IntField,
		pub volume: FloatField,
		pub pitch: FloatField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x4a]
	pub struct PlayerlistHeader {
		pub header: StringField,
		pub footer: StringField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x4b]
	pub struct Collect {
		pub collected_entity_id: VarIntField,
		pub collector_entity_id: VarIntField,
		pub pickup_item_count: VarIntField,
	}


	#[derive(ClientBoundPacket)]
	#[packet_id = 0x4c]
	pub struct EntityTeleport {
		pub entity_id: VarIntField,
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub yaw: ByteField,
		pub pitch: ByteField,
		pub on_ground: BoolField,
	}


/* TODO incomplete struct Advancements
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x4d]
	pub struct Advancements {
		pub reset: BoolField,
		// TODO pub advancement_mapping: Array { count_ty: Varint },
		// TODO pub identifiers: Array { count_ty: Varint },
		// TODO pub progress_mapping: Array { count_ty: Varint },
	}
*/

/* TODO incomplete struct EntityUpdateAttributes
	#[derive(ClientBoundPacket)]
	#[packet_id = 0x4e]
	pub struct EntityUpdateAttributes {
		pub entity_id: VarIntField,
		// TODO pub properties: Array { count_ty: I32 },
	}
*/

	#[derive(ClientBoundPacket)]
	#[packet_id = 0x4f]
	pub struct EntityEffect {
		pub entity_id: VarIntField,
		pub effect_id: ByteField,
		pub amplifier: ByteField,
		pub duration: VarIntField,
		pub hide_particles: ByteField,
	}


}

pub mod server {
	use super::*;

	#[derive(ServerBoundPacket)]
	#[packet_id = 0x00]
	pub struct TeleportConfirm {
		pub teleport_id: VarIntField,
	}


/* TODO incomplete struct TabComplete
	#[derive(ServerBoundPacket)]
	#[packet_id = 0x01]
	pub struct TabComplete {
		pub text: StringField,
		pub assume_command: BoolField,
		// TODO pub looked_at_block: Option(Position),
	}
*/

	#[derive(ServerBoundPacket)]
	#[packet_id = 0x02]
	pub struct Chat {
		pub message: StringField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x03]
	pub struct ClientCommand {
		pub action_id: VarIntField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x04]
	pub struct Settings {
		pub locale: StringField,
		pub view_distance: ByteField,
		pub chat_flags: VarIntField,
		pub chat_colors: BoolField,
		pub skin_parts: UByteField,
		pub main_hand: VarIntField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x05]
	pub struct Transaction {
		pub window_id: ByteField,
		pub action: ShortField,
		pub accepted: BoolField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x06]
	pub struct EnchantItem {
		pub window_id: ByteField,
		pub enchantment: ByteField,
	}


/* TODO incomplete struct WindowClick
	#[derive(ServerBoundPacket)]
	#[packet_id = 0x07]
	pub struct WindowClick {
		pub window_id: UByteField,
		pub slot: ShortField,
		pub mouse_button: ByteField,
		pub action: ShortField,
		pub mode: ByteField,
		// TODO pub item: Slot,
	}
*/

	#[derive(ServerBoundPacket)]
	#[packet_id = 0x08]
	pub struct CloseWindow {
		pub window_id: UByteField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x09]
	pub struct CustomPayload {
		pub channel: StringField,
		pub data: RestOfPacketByteArrayField,
	}


/* TODO incomplete struct UseEntity
	#[derive(ServerBoundPacket)]
	#[packet_id = 0x0a]
	pub struct UseEntity {
		pub target: VarIntField,
		pub mouse: VarIntField,
		// TODO pub x: Switch,
		// TODO pub y: Switch,
		// TODO pub z: Switch,
		// TODO pub hand: Switch,
	}
*/

	#[derive(ServerBoundPacket)]
	#[packet_id = 0x0b]
	pub struct KeepAlive {
		pub keep_alive_id: LongField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x0c]
	pub struct Flying {
		pub on_ground: BoolField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x0d]
	pub struct Position {
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub on_ground: BoolField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x0e]
	pub struct PositionLook {
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub yaw: FloatField,
		pub pitch: FloatField,
		pub on_ground: BoolField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x0f]
	pub struct Look {
		pub yaw: FloatField,
		pub pitch: FloatField,
		pub on_ground: BoolField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x10]
	pub struct VehicleMove {
		pub x: DoubleField,
		pub y: DoubleField,
		pub z: DoubleField,
		pub yaw: FloatField,
		pub pitch: FloatField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x11]
	pub struct SteerBoat {
		pub left_paddle: BoolField,
		pub right_paddle: BoolField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x12]
	pub struct CraftRecipeRequest {
		pub window_id: ByteField,
		pub recipe: VarIntField,
		pub make_all: BoolField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x13]
	pub struct Abilities {
		pub flags: ByteField,
		pub flying_speed: FloatField,
		pub walking_speed: FloatField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x14]
	pub struct BlockDig {
		pub status: ByteField,
		pub location: PositionField<340>,
		pub face: ByteField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x15]
	pub struct EntityAction {
		pub entity_id: VarIntField,
		pub action_id: VarIntField,
		pub jump_boost: VarIntField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x16]
	pub struct SteerVehicle {
		pub sideways: FloatField,
		pub forward: FloatField,
		pub jump: UByteField,
	}


/* TODO incomplete struct CraftingBookData
	#[derive(ServerBoundPacket)]
	#[packet_id = 0x17]
	pub struct CraftingBookData {
		pub r#type: VarIntField,
		// TODO pub anon: Switch,
	}
*/

	#[derive(ServerBoundPacket)]
	#[packet_id = 0x18]
	pub struct ResourcePackReceive {
		pub result: VarIntField,
	}


/* TODO incomplete struct AdvancementTab
	#[derive(ServerBoundPacket)]
	#[packet_id = 0x19]
	pub struct AdvancementTab {
		pub action: VarIntField,
		// TODO pub tab_id: Switch,
	}
*/

	#[derive(ServerBoundPacket)]
	#[packet_id = 0x1a]
	pub struct HeldItemSlot {
		pub slot_id: ShortField,
	}


/* TODO incomplete struct SetCreativeSlot
	#[derive(ServerBoundPacket)]
	#[packet_id = 0x1b]
	pub struct SetCreativeSlot {
		pub slot: ShortField,
		// TODO pub item: Slot,
	}
*/

	#[derive(ServerBoundPacket)]
	#[packet_id = 0x1c]
	pub struct UpdateSign {
		pub location: PositionField<340>,
		pub text_1: StringField,
		pub text_2: StringField,
		pub text_3: StringField,
		pub text_4: StringField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x1d]
	pub struct ArmAnimation {
		pub hand: VarIntField,
	}


/* TODO incomplete struct Spectate
	#[derive(ServerBoundPacket)]
	#[packet_id = 0x1e]
	pub struct Spectate {
		// TODO pub target: Uuid,
	}
*/

	#[derive(ServerBoundPacket)]
	#[packet_id = 0x1f]
	pub struct BlockPlace {
		pub location: PositionField<340>,
		pub direction: VarIntField,
		pub hand: VarIntField,
		pub cursor_x: FloatField,
		pub cursor_y: FloatField,
		pub cursor_z: FloatField,
	}


	#[derive(ServerBoundPacket)]
	#[packet_id = 0x20]
	pub struct UseItem {
		pub hand: VarIntField,
	}


}

