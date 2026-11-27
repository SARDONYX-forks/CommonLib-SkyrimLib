use core::ffi::c_void;

use core_util::inherit;

use crate::offsets::offsets_rtti::{RTTI_TES, RTTI_TES__SystemEventAdapter};
use crate::offsets::offsets_vtable::{VTABLE_TES, VTABLE_TES__SystemEventAdapter};
use crate::re::{
    ArchiveStreamOpenedEvent, BSContainerForEachResult, BSSimpleList, BSSystemEvent, BSTEventSink,
    BSTTuple, BSTempNodeManager, GridCellArray, ICellAttachDetachEventSource,
    ImageSpaceModifierInstance, MATERIAL_ID, NavMeshInfoMap, NiAVObject, NiDirectionalLight,
    NiFogProperty, NiNode, NiPoint2, NiPoint3, NiPointer, PlayerCharacter, PositionPlayerEvent,
    Sky, TESLandTexture, TESNPC, TESObjectCELL, TESObjectREFR, TESWorldSpace, bhkPickData,
};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};
use crate::version::RUNTIME_SSE_1_6_1130;

/// C++ `RE::TES::SystemEventAdapter`
#[repr(C)]
pub struct TESSystemEventAdapter {
    pub base: BSTEventSink<BSSystemEvent>, // 00
    pub unk08: u64,                        // 08
}

const _: () = assert!(core::mem::size_of::<TESSystemEventAdapter>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESSystemEventAdapter, unk08) == 0x08);

inherit!(TESSystemEventAdapter => BSTEventSink<BSSystemEvent>, base);

impl RttiType for TESSystemEventAdapter {
    // TODO: SOURCE - `RE/T/TES.h` still names this nested class with
    // `RTTI_TES`/`VTABLE_TES`, but generated offsets expose dedicated
    // `RTTI_TES__SystemEventAdapter` / `VTABLE_TES__SystemEventAdapter`
    // entries. Keep following the generated runtime data until the upstream
    // header annotation is reconciled.
    const RTTI: VariantID = RTTI_TES__SystemEventAdapter;
}

impl TESSystemEventAdapter {
    pub const RTTI: VariantID = RTTI_TES__SystemEventAdapter;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TES__SystemEventAdapter;

    // override (BSTEventSink<BSSystemEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;  // 01
}

/// C++ `RE::TES::ParticleObjectCache`
#[repr(C)]
pub struct TESParticleObjectCache {
    pub model: crate::re::NiPointer<crate::re::NiAVObject>, // 00
    pub clones: crate::re::NiPointer<crate::re::NiAVObject>, // 08
    pub next: *mut TESParticleObjectCache,                  // 10
}

const _: () = assert!(core::mem::size_of::<TESParticleObjectCache>() == 0x18);

/// C++ `RE::TES::RUNTIME_DATA`
#[repr(C)]
pub struct TESRuntimeData {
    pub unk128: u8,                  // 00
    pub show_land_borders: bool,     // 01
    pub unk12a: u8,                  // 02
    pub unk12b: u8,                  // 03
    pub unk12c: u8,                  // 04
    pub unk12d: u8,                  // 05
    pub unk12e: u8,                  // 06
    pub unk12f: u8,                  // 07
    pub unk130: u16,                 // 08
    pub unk132: u8,                  // 0A
    pub unk133: u8,                  // 0B
    pub grid_buffer_distance_x: f32, // 0C (134)
    pub grid_buffer_distance_y: f32, // 10 (138)
    pub allow_unused_purge: bool,    // 14 (13C)
    pub pad15: [u8; 3],              // 15
}

const _: () = assert!(core::mem::size_of::<TESRuntimeData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(TESRuntimeData, show_land_borders) == 0x01);
const _: () = assert!(core::mem::offset_of!(TESRuntimeData, grid_buffer_distance_x) == 0x0C);
const _: () = assert!(core::mem::offset_of!(TESRuntimeData, allow_unused_purge) == 0x14);

/// C++ `RE::TES::AE_RUNTIME_DATA`
#[repr(C)]
pub struct TESAERuntimeData {
    pub unk128: u32,                 // 00
    pub land_border_mode: u32,       // 04
    pub border_color_agbr: u32,      // 08
    pub grid_buffer_distance_x: f32, // 0C (134)
    pub grid_buffer_distance_y: f32, // 10 (138)
    pub allow_unused_purge: bool,    // 14 (13C)
    pub pad15: [u8; 3],              // 15
    pub unk140: u64,                 // 18
}

const _: () = assert!(core::mem::size_of::<TESAERuntimeData>() == 0x20);
const _: () = assert!(core::mem::offset_of!(TESAERuntimeData, land_border_mode) == 0x04);
const _: () = assert!(core::mem::offset_of!(TESAERuntimeData, border_color_agbr) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESAERuntimeData, allow_unused_purge) == 0x14);
const _: () = assert!(core::mem::offset_of!(TESAERuntimeData, unk140) == 0x18);

/// C++ `RE::TES::RUNTIME_DATA2`
#[repr(C)]
pub struct TESRuntimeData2 {
    pub world_space: *mut TESWorldSpace, // 00
    pub dead_count: BSSimpleList<*mut BSTTuple<*mut TESNPC, u16>>, // 08
    /// Ref-counted; queues facegen addon-node model/texture swaps.
    pub queued_face_gen_addon_handle: *mut crate::re::QueuedFile, // 18 (158)
    /// Lazily loaded `Effects\blooddecal.dds`.
    pub blood_decal_texture: crate::re::NiPointer<crate::re::NiSourceTexture>, // 20 (160)
    /// Ref-counted; purpose beyond the handle itself unresolved.
    pub queued_form_data_handle: *mut crate::re::QueuedFile, // 28 (168)
    /// Ref-counted; registers blood-spray/water-splash NIFs.
    pub queued_blood_spray_water_splash_handle: *mut crate::re::QueuedFile, // 30 (170)
    pub unk178: u64,                     // 38
    pub unk180: u64,                     // 40
    pub unk188: u64,                     // 48
    pub unk190: u64,                     // 50
    pub unk198: u64,                     // 58
    pub unk1a0: u64,                     // 60
    pub unk1a8: u64,                     // 68
    pub unk1b0: u64,                     // 70
    pub unk1b8: u64,                     // 78
    pub unk1c0: u64,                     // 80
    pub unk1c8: u64,                     // 88
    pub unk1d0: u64,                     // 90
    pub unk1d8: u64,                     // 98
    pub unk1e0: u64,                     // A0
    pub unk1e8: u64,                     // A8
    pub unk1f0: u64,                     // B0
    pub unk1f8: u64,                     // B8
    pub unk200: u64,                     // C0
    pub unk208: u64,                     // C8
    pub unk210: u64,                     // D0
    pub unk218: u64,                     // D8
    pub unk220: u64,                     // E0
    pub player_character: *mut PlayerCharacter, // E8
    pub unk230: u64,                     // F0
    pub unk238: u64,                     // F8
    pub unk240: u64,                     // 100
    pub unk248: u64,                     // 108
    pub unk250: u64,                     // 110
    pub unk258: u64,                     // 118
    pub unk260: u64,                     // 120
    pub unk268: u64,                     // 128
    pub unk270: u64,                     // 130
    pub unk278: u64,                     // 138
    pub unk280: u64,                     // 140
    /// Head of a callback-keyed clone/cache linked list.
    pub particle_cache_head: *mut TESParticleObjectCache, // 148 (288)
    pub system_event_adapter: TESSystemEventAdapter, // 150
    /// Incremented and decremented as a 32-bit value.
    pub placeable_water_count: u32, // 160 (2A0)
    pub pad2a4: u32,                     // 164
    /// Lazily constructed on first access.
    pub nav_mesh_info_map: *mut NavMeshInfoMap, // 168
    // TODO: `LoadedAreaBound` is not translated; this is a ref-counted
    // `NiPointer<LoadedAreaBound>` kept as a raw pointer until it is.
    pub loaded_area_bound: *mut c_void, // 170 (2B0)
}

const _: () = assert!(core::mem::size_of::<TESRuntimeData2>() == 0x178);
const _: () = assert!(core::mem::offset_of!(TESRuntimeData2, world_space) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESRuntimeData2, dead_count) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESRuntimeData2, player_character) == 0xE8);
const _: () = assert!(core::mem::offset_of!(TESRuntimeData2, system_event_adapter) == 0x150);
const _: () = assert!(core::mem::offset_of!(TESRuntimeData2, nav_mesh_info_map) == 0x168);

/// Honest common prefix of C++ `RE::TES`.
#[repr(C)]
pub struct TES {
    pub base: ICellAttachDetachEventSource, // 000
    pub archive_stream_opened_event_sink: BSTEventSink<ArchiveStreamOpenedEvent>, // 060
    pub position_player_event_sink: BSTEventSink<PositionPlayerEvent>, // 068
    pub unk070: u64,                        // 070
    pub grid_cells: *mut GridCellArray,     // 078
    pub obj_root: *mut NiNode,              // 080
    pub lod_land_root: *mut NiNode,         // 088
    pub obj_lod_water_root: *mut NiNode,    // 090
    pub temp_node_manager: *mut BSTempNodeManager, // 098
    pub obj_light: *mut NiDirectionalLight, // 0A0
    pub obj_fog: *mut NiFogProperty,        // 0A8
    pub current_grid_x: i32,                // 0B0
    pub current_grid_y: i32,                // 0B4
    pub current_queued_x: i32,              // 0B8
    pub current_queued_y: i32,              // 0BC
    pub interior_cell: *mut TESObjectCELL,  // 0C0
    pub interior_buffer: *mut *mut TESObjectCELL, // 0C8
    pub exterior_buffer: *mut *mut TESObjectCELL, // 0D0
    pub unk0d8: u64,                        // 0D8
    pub save_grid_x: i32,                   // 0E0
    pub save_grid_y: i32,                   // 0E4
    pub unk0e8: u64,                        // 0E8
    pub unk0f0: u64,                        // 0F0
    pub unk0f8: u64,                        // 0F8
    pub sky: *mut Sky,                      // 100
    pub active_image_space_modifiers: BSSimpleList<NiPointer<ImageSpaceModifierInstance>>, // 108
    pub unk118: u64,                        // 118
    pub unk120: u64,                        // 120
}

const _: () = assert!(core::mem::size_of::<TES>() == 0x128);
const _: () = assert!(core::mem::offset_of!(TES, archive_stream_opened_event_sink) == 0x60);
const _: () = assert!(core::mem::offset_of!(TES, position_player_event_sink) == 0x68);
const _: () = assert!(core::mem::offset_of!(TES, grid_cells) == 0x78);
const _: () = assert!(core::mem::offset_of!(TES, interior_cell) == 0x0C0);
const _: () = assert!(core::mem::offset_of!(TES, active_image_space_modifiers) == 0x108);
const _: () = assert!(core::mem::offset_of!(TES, unk120) == 0x120);

inherit!(TES : ICellAttachDetachEventSource);
inherit!(
    TES => BSTEventSink<ArchiveStreamOpenedEvent>,
    archive_stream_opened_event_sink
);
inherit!(TES => BSTEventSink<PositionPlayerEvent>, position_player_event_sink);

impl RttiType for TES {
    const RTTI: VariantID = RTTI_TES;
}

impl TES {
    pub const RTTI: VariantID = RTTI_TES;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TES;
    pub const COMMON_SIZE: usize = 0x128;
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x2B8, 0x2C0, 0x2B8);
    pub const RUNTIME_DATA_OFFSET: usize = 0x128;
    pub const RUNTIME_DATA2_OLDER_OFFSET: usize = 0x140;
    pub const RUNTIME_DATA2_NEWER_OFFSET: usize = 0x148;

    crate::runtime_optional_data_accessor! {
        fn runtime_data_impl() -> TESRuntimeData {
            version: RUNTIME_SSE_1_6_1130,
            older: Self::RUNTIME_DATA_OFFSET,
            newer: 0x0
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        fn runtime_data_impl_mut() -> TESRuntimeData {
            version: RUNTIME_SSE_1_6_1130,
            older: Self::RUNTIME_DATA_OFFSET,
            newer: 0x0
        }
    }

    // TODO: SOURCE - CommonLib's `TES::GetAERuntimeData()` relocates from
    // `this + 0x120`, but `AE_RUNTIME_DATA_CONTENT` is documented at object
    // offsets `0x128..0x147` and `STATIC_ASSERT_SIZE(TES, ..., 0x128, 0x128)`
    // marks the honest cross-runtime prefix through `0x127`. Keep the named AE
    // tail accessor at `0x128` until that `0x120..0x127` overlap is
    // RE-confirmed.
    crate::runtime_optional_data_accessor! {
        fn ae_runtime_data_impl() -> TESAERuntimeData {
            version: RUNTIME_SSE_1_6_1130,
            older: 0x0,
            newer: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        fn ae_runtime_data_impl_mut() -> TESAERuntimeData {
            version: RUNTIME_SSE_1_6_1130,
            older: 0x0,
            newer: Self::RUNTIME_DATA_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        fn runtime_data2_impl() -> TESRuntimeData2 {
            version: RUNTIME_SSE_1_6_1130,
            older: Self::RUNTIME_DATA2_OLDER_OFFSET,
            newer: Self::RUNTIME_DATA2_NEWER_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        fn runtime_data2_impl_mut() -> TESRuntimeData2 {
            version: RUNTIME_SSE_1_6_1130,
            older: Self::RUNTIME_DATA2_OLDER_OFFSET,
            newer: Self::RUNTIME_DATA2_NEWER_OFFSET
        }
    }

    // override (BSTEventSink<BSResource::ArchiveStreamOpenedEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;  // 01

    // override (BSTEventSink<PositionPlayerEvent>)
    // BSEventNotifyControl ProcessEvent(...) override;  // 01

    crate::relocation_variable! {
        fn singleton_ptr() -> *mut TES => RelocationID::new(516923, 403450), is_indirect_ptr
    }

    crate::relocation_func! {
        pub fn get_cell(&self, position: &NiPoint3) -> *mut TESObjectCELL => RelocationID::new(13177, 13322)
    }

    crate::relocation_func! {
        pub fn get_land_material_type(&self, position: &NiPoint3) -> MATERIAL_ID => RelocationID::new(13203, 13349)
    }

    crate::relocation_func! {
        pub fn get_land_height(&mut self, position: &NiPoint3, height_out: &mut f32) -> bool => RelocationID::new(13198, 13344)
    }

    crate::relocation_func! {
        pub fn get_land_texture(&self, position: &NiPoint3) -> *mut TESLandTexture => RelocationID::new(13202, 13348)
    }

    crate::relocation_variable! {
        pub fn get_grids_to_load_copy() -> &'static mut i32 => RelocationID::new(527644, 414596)
    }

    crate::relocation_func! {
        pub fn cancel_master_file_loads(&mut self) => RelocationID::new(13188, 13333)
    }

    crate::relocation_func! {
        pub fn resume_master_file_loads(&mut self) => RelocationID::new(13189, 13334)
    }

    crate::relocation_func! {
        fn set_current_cell_impl(
            &mut self,
            cell: *mut TESObjectCELL,
            position: *const crate::re::NiPoint3,
            unk4: f32
        ) => RelocationID::new(13171, 13316)
    }

    /// Core interior/exterior cell-transition engine backing every cell
    /// attach/detach (interior enter/exit, coc, worldspace load).
    #[inline(always)]
    pub fn set_current_cell(
        &mut self,
        cell: *mut TESObjectCELL,
        position: &crate::re::NiPoint3,
        unk4: f32,
    ) {
        self.set_current_cell_impl(cell, position, unk4);
    }

    crate::relocation_func! {
        pub fn get_water_height(&self, pos: &NiPoint3, cell: *mut TESObjectCELL) -> f32 => crate::relocation::VersionedRelocationID::new(crate::version::RUNTIME_SSE_1_7_99, 13212, 13358, 523463)
    }

    crate::relocation_func! {
        pub fn pick(&mut self, pick_data: &mut bhkPickData) -> *mut NiAVObject => RelocationID::new(13221, 13371)
    }

    crate::relocation_func! {
        pub fn purge_buffered_cells(&mut self) => RelocationID::new(13159, 13299)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut TES {
        Self::singleton_ptr()
    }

    #[inline(always)]
    pub fn get_runtime_data(&self) -> Option<&TESRuntimeData> {
        crate::runtime_assert_size!(TESRuntimeData, version: RUNTIME_SSE_1_6_1130, older: 0x18, newer: 0x0);
        crate::runtime_assert_offset!(TESRuntimeData, show_land_borders, version: RUNTIME_SSE_1_6_1130, older: 0x01, newer: 0x0);
        crate::runtime_assert_offset!(TESRuntimeData, allow_unused_purge, version: RUNTIME_SSE_1_6_1130, older: 0x14, newer: 0x0);
        self.runtime_data_impl()
    }

    #[inline(always)]
    pub fn get_runtime_data_mut(&mut self) -> Option<&mut TESRuntimeData> {
        crate::runtime_assert_size!(TESRuntimeData, version: RUNTIME_SSE_1_6_1130, older: 0x18, newer: 0x0);
        self.runtime_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_ae_runtime_data(&self) -> Option<&TESAERuntimeData> {
        crate::runtime_assert_size!(TESAERuntimeData, version: RUNTIME_SSE_1_6_1130, older: 0x0, newer: 0x20);
        crate::runtime_assert_offset!(TESAERuntimeData, land_border_mode, version: RUNTIME_SSE_1_6_1130, older: 0x0, newer: 0x04);
        crate::runtime_assert_offset!(TESAERuntimeData, border_color_agbr, version: RUNTIME_SSE_1_6_1130, older: 0x0, newer: 0x08);
        self.ae_runtime_data_impl()
    }

    #[inline(always)]
    pub fn get_ae_runtime_data_mut(&mut self) -> Option<&mut TESAERuntimeData> {
        crate::runtime_assert_size!(TESAERuntimeData, version: RUNTIME_SSE_1_6_1130, older: 0x0, newer: 0x20);
        self.ae_runtime_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_runtime_data2(&self) -> &TESRuntimeData2 {
        crate::runtime_assert_size!(TESRuntimeData2, se: 0x178, ae: 0x178, vr: 0x178);
        crate::runtime_assert_offset!(TESRuntimeData2, world_space, se: 0x00, ae: 0x00, vr: 0x00);
        crate::runtime_assert_offset!(TESRuntimeData2, player_character, se: 0xE8, ae: 0xE8, vr: 0xE8);
        crate::runtime_assert_offset!(TESRuntimeData2, system_event_adapter, se: 0x150, ae: 0x150, vr: 0x150);
        crate::runtime_assert_offset!(TESRuntimeData2, nav_mesh_info_map, se: 0x168, ae: 0x168, vr: 0x168);
        self.runtime_data2_impl()
    }

    #[inline(always)]
    pub fn get_runtime_data2_mut(&mut self) -> &mut TESRuntimeData2 {
        crate::runtime_assert_size!(TESRuntimeData2, se: 0x178, ae: 0x178, vr: 0x178);
        self.runtime_data2_impl_mut()
    }

    pub fn for_each_cell<F>(&self, mut callback: F)
    where
        F: FnMut(*mut TESObjectCELL),
    {
        if !self.interior_cell.is_null() {
            callback(self.interior_cell);
        } else if let Some(grid_cells) = unsafe { self.grid_cells.as_ref() } {
            let grid_length = grid_cells.base.length;
            if grid_length > 0 {
                let mut x = 0;
                while x < grid_length {
                    let mut y = 0;
                    while y < grid_length {
                        let cell = grid_cells.get_cell(x, y);
                        if let Some(cell_ref) = unsafe { cell.as_ref() } {
                            if cell_ref.is_attached() {
                                callback(cell);
                            }
                        }
                        y += 1;
                    }
                    x += 1;
                }
            }
        }

        if let Some(world_space) = unsafe { self.get_runtime_data2().world_space.as_ref() } {
            let sky_cell = world_space.get_sky_cell();
            if !sky_cell.is_null() {
                callback(sky_cell);
            }
        }
    }

    pub fn for_each_cell_in_range<F>(
        &self,
        origin: *mut TESObjectREFR,
        radius: f32,
        mut callback: F,
    ) where
        F: FnMut(*mut TESObjectCELL),
    {
        if origin.is_null() || radius <= 0.0 {
            self.for_each_cell(callback);
            return;
        }

        let origin_pos = unsafe { (*origin).get_position() };

        if !self.interior_cell.is_null() {
            callback(self.interior_cell);
        } else if let Some(grid_cells) = unsafe { self.grid_cells.as_ref() } {
            let grid_length = grid_cells.base.length;
            if grid_length > 0 {
                let y_plus = origin_pos.y + radius;
                let y_minus = origin_pos.y - radius;
                let x_plus = origin_pos.x + radius;
                let x_minus = origin_pos.x - radius;

                let mut x = 0;
                while x < grid_length {
                    let mut y = 0;
                    while y < grid_length {
                        let cell = grid_cells.get_cell(x, y);
                        if let Some(cell_ref) = unsafe { cell.as_ref() } {
                            if cell_ref.is_attached() {
                                let cell_coords = cell_ref.get_coordinates();
                                if let Some(cell_coords) = unsafe { cell_coords.as_ref() } {
                                    let world_pos =
                                        NiPoint2::new(cell_coords.world_x, cell_coords.world_y);
                                    if world_pos.x < x_plus
                                        && (world_pos.x + 4096.0) > x_minus
                                        && world_pos.y < y_plus
                                        && (world_pos.y + 4096.0) > y_minus
                                    {
                                        callback(cell);
                                        return;
                                    }
                                }
                            }
                        }
                        y += 1;
                    }
                    x += 1;
                }
            }
        }

        if let Some(world_space) = unsafe { self.get_runtime_data2().world_space.as_ref() } {
            let sky_cell = world_space.get_sky_cell();
            if !sky_cell.is_null() {
                callback(sky_cell);
            }
        }
    }

    pub fn for_each_reference<F>(&self, mut callback: F)
    where
        F: FnMut(*mut TESObjectREFR) -> BSContainerForEachResult,
    {
        if let Some(interior_cell) = unsafe { self.interior_cell.as_ref() } {
            interior_cell.for_each_reference(|reference| callback(reference));
        } else if let Some(grid_cells) = unsafe { self.grid_cells.as_ref() } {
            let grid_length = grid_cells.base.length;
            if grid_length > 0 {
                let mut x = 0;
                while x < grid_length {
                    let mut y = 0;
                    while y < grid_length {
                        let cell = grid_cells.get_cell(x, y);
                        if let Some(cell_ref) = unsafe { cell.as_ref() } {
                            if cell_ref.is_attached() {
                                cell_ref.for_each_reference(|reference| callback(reference));
                            }
                        }
                        y += 1;
                    }
                    x += 1;
                }
            }
        }

        if let Some(world_space) = unsafe { self.get_runtime_data2().world_space.as_ref() } {
            let sky_cell = world_space.get_sky_cell();
            if let Some(sky_cell) = unsafe { sky_cell.as_ref() } {
                sky_cell.for_each_reference(|reference| callback(reference));
            }
        }
    }

    pub fn for_each_reference_in_range<F>(
        &self,
        origin: *mut TESObjectREFR,
        radius: f32,
        mut callback: F,
    ) where
        F: FnMut(*mut TESObjectREFR) -> BSContainerForEachResult,
    {
        if origin.is_null() || radius <= 0.0 {
            self.for_each_reference(callback);
            return;
        }

        let origin_pos = unsafe { (*origin).get_position() };

        if let Some(interior_cell) = unsafe { self.interior_cell.as_ref() } {
            interior_cell
                .for_each_reference_in_range(&origin_pos, radius, |reference| callback(reference));
        } else if let Some(grid_cells) = unsafe { self.grid_cells.as_ref() } {
            let grid_length = grid_cells.base.length;
            if grid_length > 0 {
                let y_plus = origin_pos.y + radius;
                let y_minus = origin_pos.y - radius;
                let x_plus = origin_pos.x + radius;
                let x_minus = origin_pos.x - radius;

                let mut x = 0;
                while x < grid_length {
                    let mut y = 0;
                    while y < grid_length {
                        let cell = grid_cells.get_cell(x, y);
                        if let Some(cell_ref) = unsafe { cell.as_ref() } {
                            if cell_ref.is_attached() {
                                let cell_coords = cell_ref.get_coordinates();
                                if let Some(cell_coords) = unsafe { cell_coords.as_ref() } {
                                    let world_pos =
                                        NiPoint2::new(cell_coords.world_x, cell_coords.world_y);
                                    if world_pos.x < x_plus
                                        && (world_pos.x + 4096.0) > x_minus
                                        && world_pos.y < y_plus
                                        && (world_pos.y + 4096.0) > y_minus
                                    {
                                        cell_ref.for_each_reference_in_range(
                                            &origin_pos,
                                            radius,
                                            |reference| callback(reference),
                                        );
                                    }
                                }
                            }
                        }
                        y += 1;
                    }
                    x += 1;
                }
            }
        }

        if let Some(world_space) = unsafe { self.get_runtime_data2().world_space.as_ref() } {
            let sky_cell = world_space.get_sky_cell();
            if let Some(sky_cell) = unsafe { sky_cell.as_ref() } {
                sky_cell.for_each_reference_in_range(&origin_pos, radius, |reference| {
                    callback(reference)
                });
            }
        }
    }
}
