#![doc(html_root_url = "http://arcnmx.github.io/ddcutil-rs/")]
#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

use libc::{FILE, c_char, c_int, c_void};
use std::fmt;
use std::slice::from_raw_parts;

#[link(name = "ddcutil")]
unsafe extern "C" {
    // Library build information
    pub fn ddca_ddcutil_version() -> DDCA_Ddcutil_Version_Spec;
    pub fn ddca_ddcutil_version_string() -> *const c_char;
    pub fn ddca_ddcutil_extended_version_string() -> *const c_char;
    pub fn ddca_build_options() -> DDCA_Build_Option_Flags;
    pub fn ddca_libddcutil_filename() -> *const c_char;

    // Error Detail
    pub fn ddca_get_error_detail() -> *mut DDCA_Error_Detail;
    pub fn ddca_free_error_detail(ddca_erec: *mut DDCA_Error_Detail);
    pub fn ddca_report_error_detail(ddca_erec: *mut DDCA_Error_Detail, depth: c_int);

    // Initialization
    pub fn ddca_init(
        libopts: *const c_char,
        syslog_level: DDCA_Syslog_Level,
        opts: DDCA_Init_Options,
    ) -> DDCA_Status;
    pub fn ddca_init2(
        libopts: *const c_char,
        syslog_level_arg: DDCA_Syslog_Level,
        opts: DDCA_Init_Options,
        infomsg_loc: *mut *mut *mut c_char,
    ) -> DDCA_Status;

    // Status Codes
    pub fn ddca_rc_name(status_code: DDCA_Status) -> *const c_char;
    pub fn ddca_rc_desc(status_code: DDCA_Status) -> *const c_char;

    // Global Settings
    pub fn ddca_enable_verify(onoff: bool) -> bool;
    pub fn ddca_is_verify_enabled() -> bool;

    // Performance
    pub fn ddca_set_sleep_multiplier(multiplier: f64) -> f64;
    pub fn ddca_get_sleep_multiplier() -> f64;
    pub fn ddca_set_display_sleep_multiplier(
        dref: DDCA_Display_Ref,
        multiplier: DDCA_Sleep_Multiplier,
    ) -> DDCA_Status;
    pub fn ddca_get_current_display_sleep_multiplier(
        dref: DDCA_Display_Ref,
        multiplier_loc: *mut DDCA_Sleep_Multiplier,
    ) -> DDCA_Status;
    pub fn ddca_enable_dynamic_sleep(onoff: bool) -> bool;
    pub fn ddca_is_dynamic_sleep_enabled() -> bool;

    // Output Redirection
    pub fn ddca_set_fout(fout: *mut FILE);
    pub fn ddca_set_fout_to_default();
    pub fn ddca_set_ferr(ferr: *mut FILE);
    pub fn ddca_set_ferr_to_default();

    // Utility functions for capturing output
    pub fn ddca_start_capture(flags: DDCA_Capture_Option_Flags);
    pub fn ddca_end_capture() -> *mut c_char;

    // Message Control
    pub fn ddca_get_output_level() -> DDCA_Output_Level;
    pub fn ddca_set_output_level(newval: DDCA_Output_Level) -> DDCA_Output_Level;
    pub fn ddca_output_level_name(val: DDCA_Output_Level) -> *mut c_char;
    pub fn ddca_syslog_level_from_name(name: *const c_char) -> DDCA_Syslog_Level;

    // Statistics and Diagnostics
    pub fn ddca_reset_stats();
    pub fn ddca_show_stats(stats: DDCA_Stats_Type, include_per_display_data: bool, depth: c_int);
    pub fn ddca_report_locks(depth: c_int);
    // Display Detection
    pub fn ddca_get_display_refs(
        include_invalid_displays: bool,
        drefs_loc: *mut *mut DDCA_Display_Ref,
    ) -> DDCA_Status;
    pub fn ddca_get_display_info(
        ddca_dref: DDCA_Display_Ref,
        dinfo_loc: *mut *mut DDCA_Display_Info,
    ) -> DDCA_Status;
    pub fn ddca_free_display_info(info_rec: *mut DDCA_Display_Info);
    pub fn ddca_get_display_info2(
        ddca_dref: DDCA_Display_Ref,
        dinfo_loc: *mut *mut DDCA_Display_Info2,
    ) -> DDCA_Status;
    pub fn ddca_free_display_info2(info_rec: *mut DDCA_Display_Info2);
    pub fn ddca_get_display_info_list2(
        include_invalid_displays: bool,
        dlist_loc: *mut *mut DDCA_Display_Info_List,
    ) -> DDCA_Status;
    pub fn ddca_free_display_info_list(dlist: *mut DDCA_Display_Info_List);
    pub fn ddca_report_display_info(dinfo: *mut DDCA_Display_Info, depth: c_int) -> DDCA_Status;
    pub fn ddca_report_display_info2(dinfo: *mut DDCA_Display_Info2, depth: c_int) -> DDCA_Status;
    pub fn ddca_report_display_info_list(dlist: *mut DDCA_Display_Info_List, depth: c_int);
    pub fn ddca_report_displays(include_invalid_displays: bool, depth: c_int) -> c_int;
    pub fn ddca_redetect_displays() -> DDCA_Status;

    // Display Identifier
    pub fn ddca_create_dispno_display_identifier(
        dispno: c_int,
        did_loc: *mut DDCA_Display_Identifier,
    ) -> DDCA_Status;
    pub fn ddca_create_busno_display_identifier(
        busno: c_int,
        did_loc: *mut DDCA_Display_Identifier,
    ) -> DDCA_Status;
    pub fn ddca_create_mfg_model_sn_display_identifier(
        mfg_id: *const c_char,
        model: *const c_char,
        sn: *const c_char,
        did_loc: *mut DDCA_Display_Identifier,
    ) -> DDCA_Status;
    pub fn ddca_create_edid_display_identifier(
        edid: *const u8,
        did_loc: *mut DDCA_Display_Identifier,
    ) -> DDCA_Status;
    pub fn ddca_create_usb_display_identifier(
        bus: c_int,
        device: c_int,
        did_loc: *mut DDCA_Display_Identifier,
    ) -> DDCA_Status;
    pub fn ddca_create_usb_hiddev_display_identifier(
        hiddev_devno: c_int,
        did_loc: *mut DDCA_Display_Identifier,
    ) -> DDCA_Status;
    pub fn ddca_free_display_identifier(did: DDCA_Display_Identifier) -> DDCA_Status;
    pub fn ddca_did_repr(did: DDCA_Display_Identifier) -> *const c_char;

    // Display Reference
    pub fn ddca_create_display_ref(
        did: DDCA_Display_Identifier,
        dref_loc: *mut DDCA_Display_Ref,
    ) -> DDCA_Status;
    pub fn ddca_get_display_ref(
        did: DDCA_Display_Identifier,
        dref_loc: *mut DDCA_Display_Ref,
    ) -> DDCA_Status;
    pub fn ddca_validate_display_ref(
        dref: DDCA_Display_Ref,
        require_not_asleep: bool,
    ) -> DDCA_Status;
    pub fn ddca_dref_repr(dref: DDCA_Display_Ref) -> *const c_char;
    pub fn ddca_dbgrpt_display_ref(dref: DDCA_Display_Ref, depth: c_int);

    // Display Handle
    pub fn ddca_open_display2(
        ddca_dref: DDCA_Display_Ref,
        wait: bool,
        ddca_dh_loc: *mut DDCA_Display_Handle,
    ) -> DDCA_Status;
    pub fn ddca_close_display(ddca_dh: DDCA_Display_Handle) -> DDCA_Status;
    pub fn ddca_dh_repr(ddca_dh: DDCA_Display_Handle) -> *const c_char;
    pub fn ddca_display_ref_from_handle(ddca_dh: DDCA_Display_Handle) -> DDCA_Display_Ref;

    // Monitor Capabilities
    pub fn ddca_get_capabilities_string(
        ddca_dh: DDCA_Display_Handle,
        caps_loc: *mut *mut c_char,
    ) -> DDCA_Status;
    pub fn ddca_parse_capabilities_string(
        capabilities_string: *mut c_char,
        parsed_capabilities_loc: *mut *mut DDCA_Capabilities,
    ) -> DDCA_Status;
    pub fn ddca_free_parsed_capabilities(parsed_capabilities: *mut DDCA_Capabilities);
    pub fn ddca_report_parsed_capabilities_by_dref(
        parsed_capabilities: *mut DDCA_Capabilities,
        ddca_dref: DDCA_Display_Ref,
        depth: c_int,
    ) -> DDCA_Status;
    pub fn ddca_report_parsed_capabilities_by_dh(
        p_caps: *mut DDCA_Capabilities,
        ddca_dh: DDCA_Display_Handle,
        depth: c_int,
    ) -> DDCA_Status;
    pub fn ddca_report_parsed_capabilities(
        parsed_capabilities: *mut DDCA_Capabilities,
        depth: c_int,
    );
    pub fn ddca_feature_list_from_capabilities(
        parsed_caps: *mut DDCA_Capabilities,
    ) -> DDCA_Feature_List;

    // MCCS Version Specification
    pub fn ddca_get_mccs_version_by_dh(
        ddca_dh: DDCA_Display_Handle,
        p_vspec: *mut DDCA_MCCS_Version_Spec,
    ) -> DDCA_Status;

    // VCP Feature Metadata
    pub fn ddca_enable_udf(onoff: bool) -> bool;
    pub fn ddca_is_udf_enabled() -> bool;
    pub fn ddca_dfr_check_by_dref(ddca_dref: DDCA_Display_Ref) -> DDCA_Status;
    pub fn ddca_dfr_check_by_dh(ddca_dh: DDCA_Display_Handle) -> DDCA_Status;
    pub fn ddca_get_feature_metadata_by_vspec(
        feature_code: DDCA_Vcp_Feature_Code,
        vspec: DDCA_MCCS_Version_Spec,
        create_default_if_not_found: bool,
        meta_loc: *mut *mut DDCA_Feature_Metadata,
    ) -> DDCA_Status;
    pub fn ddca_get_feature_metadata_by_dref(
        feature_code: DDCA_Vcp_Feature_Code,
        ddca_dref: DDCA_Display_Ref,
        create_default_if_not_found: bool,
        meta_loc: *mut *mut DDCA_Feature_Metadata,
    ) -> DDCA_Status;
    pub fn ddca_get_feature_metadata_by_dh(
        feature_code: DDCA_Vcp_Feature_Code,
        ddca_dh: DDCA_Display_Handle,
        create_default_if_not_found: bool,
        meta_loc: *mut *mut DDCA_Feature_Metadata,
    ) -> DDCA_Status;
    pub fn ddca_free_feature_metadata(metadata: *mut DDCA_Feature_Metadata);
    pub fn ddca_get_feature_name(feature_code: DDCA_Vcp_Feature_Code) -> *const c_char;
    pub fn ddca_get_simple_nc_feature_value_name_by_table(
        feature_value_table: *mut DDCA_Feature_Value_Entry,
        feature_value: u8,
        value_name_loc: *mut *mut c_char,
    ) -> DDCA_Status;
    pub fn ddca_dbgrpt_feature_metadata(md: *mut DDCA_Feature_Metadata, depth: c_int);

    // Miscellaneous Monitor Specific Functions
    pub fn ddca_report_display_by_dref(dref: DDCA_Display_Ref, depth: c_int) -> DDCA_Status;

    // Feature Lists
    pub fn ddca_feature_list_id_name(feature_set_id: DDCA_Feature_Subset_Id) -> *const c_char;
    pub fn ddca_get_feature_list_by_dref(
        feature_set_id: DDCA_Feature_Subset_Id,
        dref: DDCA_Display_Ref,
        include_table_features: bool,
        feature_list_loc: *mut DDCA_Feature_List,
    ) -> DDCA_Status;
    pub fn ddca_feature_list_clear(vcplist: *mut DDCA_Feature_List);
    pub fn ddca_feature_list_add(
        vcplist: *mut DDCA_Feature_List,
        vcp_code: u8,
    ) -> DDCA_Feature_List;
    pub fn ddca_feature_list_contains(vcplist: DDCA_Feature_List, vcp_code: u8) -> bool;
    pub fn ddca_feature_list_eq(vcplist1: DDCA_Feature_List, vcplist2: DDCA_Feature_List) -> bool;
    pub fn ddca_feature_list_or(
        vcplist1: DDCA_Feature_List,
        vcplist2: DDCA_Feature_List,
    ) -> DDCA_Feature_List;
    pub fn ddca_feature_list_and(
        vcplist1: DDCA_Feature_List,
        vcplist2: DDCA_Feature_List,
    ) -> DDCA_Feature_List;
    pub fn ddca_feature_list_and_not(
        vcplist1: DDCA_Feature_List,
        vcplist2: DDCA_Feature_List,
    ) -> DDCA_Feature_List;
    pub fn ddca_feature_list_count(feature_list: DDCA_Feature_List) -> c_int;
    pub fn ddca_feature_list_string(
        feature_list: DDCA_Feature_List,
        value_prefix: *const c_char,
        sepstr: *const c_char,
    ) -> *const c_char;

    // Free VCP Feature Value
    pub fn ddca_free_table_vcp_value(table_value: *mut DDCA_Table_Vcp_Value);
    pub fn ddca_free_any_vcp_value(valrec: *mut DDCA_Any_Vcp_Value);

    // Get VCP Feature Value
    pub fn ddca_get_non_table_vcp_value(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        valrec: *mut DDCA_Non_Table_Vcp_Value,
    ) -> DDCA_Status;
    pub fn ddca_get_table_vcp_value(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        table_value_loc: *mut *mut DDCA_Table_Vcp_Value,
    ) -> DDCA_Status;
    pub fn ddca_get_any_vcp_value_using_explicit_type(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        value_type: DDCA_Vcp_Value_Type,
        valrec_loc: *mut *mut DDCA_Any_Vcp_Value,
    ) -> DDCA_Status;
    pub fn ddca_get_any_vcp_value_using_implicit_type(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        valrec_loc: *mut *mut DDCA_Any_Vcp_Value,
    ) -> DDCA_Status;
    pub fn ddca_format_table_vcp_value_by_dref(
        feature_code: DDCA_Vcp_Feature_Code,
        ddca_dref: DDCA_Display_Ref,
        table_value: *mut DDCA_Table_Vcp_Value,
        formatted_value_loc: *mut *mut c_char,
    ) -> DDCA_Status;
    pub fn ddca_format_non_table_vcp_value_by_dref(
        feature_code: DDCA_Vcp_Feature_Code,
        dref: DDCA_Display_Ref,
        valrec: *mut DDCA_Non_Table_Vcp_Value,
        formatted_value_loc: *mut *mut c_char,
    ) -> DDCA_Status;
    pub fn ddca_format_any_vcp_value_by_dref(
        feature_code: DDCA_Vcp_Feature_Code,
        dref: DDCA_Display_Ref,
        valrec: *mut DDCA_Any_Vcp_Value,
        formatted_value_loc: *mut *mut c_char,
    ) -> DDCA_Status;

    // Set VCP value
    pub fn ddca_set_non_table_vcp_value(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        hi_byte: u8,
        lo_byte: u8,
    ) -> DDCA_Status;
    pub fn ddca_set_table_vcp_value(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        new_value: *mut DDCA_Table_Vcp_Value,
    ) -> DDCA_Status;
    pub fn ddca_set_any_vcp_value(
        ddca_dh: DDCA_Display_Handle,
        feature_code: DDCA_Vcp_Feature_Code,
        new_value: *mut DDCA_Any_Vcp_Value,
    ) -> DDCA_Status;

    // Get or set multiple values
    pub fn ddca_get_profile_related_values(
        ddca_dh: DDCA_Display_Handle,
        profile_values_string_loc: *mut *mut c_char,
    ) -> DDCA_Status;
    pub fn ddca_set_profile_related_values(
        ddca_dh: DDCA_Display_Handle,
        profile_values_string: *mut c_char,
    ) -> DDCA_Status;

    // Report display status changes
    pub fn ddca_register_display_status_callback(
        func: DDCA_Display_Status_Callback_Func,
    ) -> DDCA_Status;
    pub fn ddca_unregister_display_status_callback(
        func: DDCA_Display_Status_Callback_Func,
    ) -> DDCA_Status;
    pub fn ddca_display_event_class_name(event_class: DDCA_Display_Event_Class) -> *const c_char;
    pub fn ddca_display_event_type_name(event_type: DDCA_Display_Event_Type) -> *const c_char;
    pub fn ddca_start_watch_displays(enabled_classes: DDCA_Display_Event_Class) -> DDCA_Status;
    pub fn ddca_stop_watch_displays(wait: bool) -> DDCA_Status;
    pub fn ddca_get_active_watch_classes(classes_loc: *mut DDCA_Display_Event_Class)
    -> DDCA_Status;
    pub fn ddca_get_display_watch_settings(settings_buffer: *mut DDCA_DW_Settings) -> DDCA_Status;
    pub fn ddca_set_display_watch_settings(settings_buffer: *mut DDCA_DW_Settings) -> DDCA_Status;
}

// Basic types
pub type DDCA_Status = c_int;

pub const DDCRC_REPORTED_UNSUPPORTED: DDCA_Status = -3005;

// Build Information
pub type DDCA_Build_Option_Flags = u32;
pub const DDCA_BUILT_WITH_NONE: DDCA_Build_Option_Flags = 0x00;
pub const DDCA_BUILT_WITH_USB: DDCA_Build_Option_Flags = 0x02;
pub const DDCA_BUILT_WITH_FAILSIM: DDCA_Build_Option_Flags = 0x04;

// Error Reporting
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Error_Detail {
    pub marker: [c_char; 4],
    pub status_code: DDCA_Status,
    pub detail: *mut c_char,
    pub cause_ct: u16,
    pub causes: [*mut DDCA_Error_Detail; 0],
}

// Initialization
pub type DDCA_Init_Options = u32;
pub const DDCA_INIT_OPTIONS_NONE: DDCA_Init_Options = 0;
pub const DDCA_INIT_OPTIONS_DISABLE_CONFIG_FILE: DDCA_Init_Options = 1;
pub const DDCA_INIT_OPTIONS_CLIENT_OPENED_SYSLOG: DDCA_Init_Options = 2;
pub const DDCA_INIT_OPTIONS_ENABLE_INIT_MSGS: DDCA_Init_Options = 4;

// Message Control
pub type DDCA_Output_Level = c_int;
pub const DDCA_OL_TERSE: DDCA_Output_Level = 0x04;
pub const DDCA_OL_NORMAL: DDCA_Output_Level = 0x08;
pub const DDCA_OL_VERBOSE: DDCA_Output_Level = 0x10;
pub const DDCA_OL_VV: DDCA_Output_Level = 0x20;

// Syslog levels
pub type DDCA_Syslog_Level = c_int;
pub const DDCA_SYSLOG_NOT_SET: DDCA_Syslog_Level = -1;
pub const DDCA_SYSLOG_NEVER: DDCA_Syslog_Level = 0;
pub const DDCA_SYSLOG_ERROR: DDCA_Syslog_Level = 3;
pub const DDCA_SYSLOG_WARNING: DDCA_Syslog_Level = 6;
pub const DDCA_SYSLOG_NOTICE: DDCA_Syslog_Level = 9;
pub const DDCA_SYSLOG_INFO: DDCA_Syslog_Level = 12;
pub const DDCA_SYSLOG_VERBOSE: DDCA_Syslog_Level = 15;
pub const DDCA_SYSLOG_DEBUG: DDCA_Syslog_Level = 18;

// Performance statistics
pub type DDCA_Stats_Type = u32;
pub const DDCA_STATS_NONE: DDCA_Stats_Type = 0x00;
pub const DDCA_STATS_TRIES: DDCA_Stats_Type = 0x01;
pub const DDCA_STATS_ERRORS: DDCA_Stats_Type = 0x02;
pub const DDCA_STATS_CALLS: DDCA_Stats_Type = 0x04;
pub const DDCA_STATS_ELAPSED: DDCA_Stats_Type = 0x08;
pub const DDCA_STATS_API: DDCA_Stats_Type = 0x10;
pub const DDCA_STATS_ALL: DDCA_Stats_Type = 0xFF;

// Miscellaneous types
pub type DDCA_Sleep_Multiplier = f64;

// Output capture
pub type DDCA_Capture_Option_Flags = u32;
pub const DDCA_CAPTURE_NOOPTS: DDCA_Capture_Option_Flags = 0;
pub const DDCA_CAPTURE_STDERR: DDCA_Capture_Option_Flags = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Ddcutil_Version_Spec {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
}

// Display Specification types
pub type DDCA_Display_Identifier = *mut c_void;
pub type DDCA_Display_Ref = *mut c_void;
pub type DDCA_Display_Handle = *mut c_void;

// VCP Feature Information
pub type DDCA_Vcp_Feature_Code = u8;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_MCCS_Version_Spec {
    pub major: u8,
    pub minor: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Feature_List {
    pub bytes: [u8; 32],
}

// Feature subset identifiers
pub type DDCA_Feature_Subset_Id = c_int;
pub const DDCA_SUBSET_UNSET: DDCA_Feature_Subset_Id = 0;
pub const DDCA_SUBSET_KNOWN: DDCA_Feature_Subset_Id = 1;
pub const DDCA_SUBSET_COLOR: DDCA_Feature_Subset_Id = 2;
pub const DDCA_SUBSET_PROFILE: DDCA_Feature_Subset_Id = 3;
pub const DDCA_SUBSET_MFG: DDCA_Feature_Subset_Id = 4;
pub const DDCA_SUBSET_CAPABILITIES: DDCA_Feature_Subset_Id = 5;
pub const DDCA_SUBSET_SCAN: DDCA_Feature_Subset_Id = 6;
pub const DDCA_SUBSET_CUSTOM: DDCA_Feature_Subset_Id = 7;

// Display Information
pub type DDCA_IO_Mode = c_int;
pub const DDCA_IO_I2C: DDCA_IO_Mode = 0;
pub const DDCA_IO_USB: DDCA_IO_Mode = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DDCA_IO_Path {
    pub io_mode: DDCA_IO_Mode,
    pub path: DDCA_IO_Path_Union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DDCA_IO_Path_Union {
    pub i2c_busno: c_int,
    pub hiddev_devno: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DDCA_Display_Info {
    pub marker: [c_char; 4],
    pub dispno: c_int,
    pub path: DDCA_IO_Path,
    pub usb_bus: c_int,
    pub usb_device: c_int,
    pub mfg_id: [c_char; 4],
    pub model_name: [c_char; 14],
    pub sn: [c_char; 14],
    pub product_code: u16,
    pub edid_bytes: [u8; 128],
    pub vcp_version: DDCA_MCCS_Version_Spec,
    pub dref: DDCA_Display_Ref,
}

pub type DDCA_Drm_Connector_Found_By = c_int;
pub const DDCA_DRM_CONNECTOR_NOT_FOUND: DDCA_Drm_Connector_Found_By = 0;
pub const DDCA_DRM_CONNECTOR_FOUND_BY_BUSNO: DDCA_Drm_Connector_Found_By = 1;
pub const DDCA_DRM_CONNECTOR_FOUND_BY_EDID: DDCA_Drm_Connector_Found_By = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DDCA_Display_Info2 {
    pub marker: [c_char; 4],
    pub dispno: c_int,
    pub path: DDCA_IO_Path,
    pub usb_bus: c_int,
    pub usb_device: c_int,
    pub mfg_id: [c_char; 4],
    pub model_name: [c_char; 14],
    pub sn: [c_char; 14],
    pub product_code: u16,
    pub edid_bytes: [u8; 128],
    pub vcp_version: DDCA_MCCS_Version_Spec,
    pub dref: DDCA_Display_Ref,
    pub drm_card_connector: [c_char; 32],
    pub drm_card_connector_found_by: DDCA_Drm_Connector_Found_By,
    pub drm_connector_id: i16,
    pub unused: [*mut c_void; 8],
}

#[repr(C)]
pub struct DDCA_Display_Info_List {
    pub ct: c_int,
    pub info: [DDCA_Display_Info; 0],
}

impl DDCA_Display_Info_List {
    pub fn info(&self) -> &[DDCA_Display_Info] {
        unsafe { from_raw_parts(self.info.as_ptr(), self.ct as usize) }
    }
}

// Version Feature Flags
pub type DDCA_Version_Feature_Flags = u16;
pub const DDCA_RO: DDCA_Version_Feature_Flags = 0x0400;
pub const DDCA_WO: DDCA_Version_Feature_Flags = 0x0200;
pub const DDCA_RW: DDCA_Version_Feature_Flags = 0x0100;
pub const DDCA_READABLE: DDCA_Version_Feature_Flags = DDCA_RO | DDCA_RW;
pub const DDCA_WRITABLE: DDCA_Version_Feature_Flags = DDCA_WO | DDCA_RW;
pub const DDCA_STD_CONT: DDCA_Version_Feature_Flags = 0x0080;
pub const DDCA_COMPLEX_CONT: DDCA_Version_Feature_Flags = 0x0040;
pub const DDCA_SIMPLE_NC: DDCA_Version_Feature_Flags = 0x0020;
pub const DDCA_EXTENDED_NC: DDCA_Version_Feature_Flags = 0x8000;
pub const DDCA_COMPLEX_NC: DDCA_Version_Feature_Flags = 0x0010;
pub const DDCA_NC_CONT: DDCA_Version_Feature_Flags = 0x0800;
pub const DDCA_WO_NC: DDCA_Version_Feature_Flags = 0x0008;
pub const DDCA_NORMAL_TABLE: DDCA_Version_Feature_Flags = 0x0004;
pub const DDCA_WO_TABLE: DDCA_Version_Feature_Flags = 0x0002;
pub const DDCA_CONT: DDCA_Version_Feature_Flags = DDCA_STD_CONT | DDCA_COMPLEX_CONT;
pub const DDCA_NC: DDCA_Version_Feature_Flags =
    DDCA_EXTENDED_NC | DDCA_SIMPLE_NC | DDCA_COMPLEX_NC | DDCA_WO_NC | DDCA_NC_CONT;
pub const DDCA_NON_TABLE: DDCA_Version_Feature_Flags = DDCA_CONT | DDCA_NC;
pub const DDCA_TABLE: DDCA_Version_Feature_Flags = DDCA_NORMAL_TABLE | DDCA_WO_TABLE;
pub const DDCA_DEPRECATED: DDCA_Version_Feature_Flags = 0x0001;

pub type DDCA_Global_Feature_Flags = u16;
pub const DDCA_SYNTHETIC_VCP_FEATURE_TABLE_ENTRY: DDCA_Global_Feature_Flags = 0x8000;
pub const DDCA_PERSISTENT_METADATA: DDCA_Global_Feature_Flags = 0x1000;
pub const DDCA_USER_DEFINED: DDCA_Global_Feature_Flags = 0x4000;
pub const DDCA_SYNTHETIC: DDCA_Global_Feature_Flags = 0x2000;

pub type DDCA_Feature_Flags = u16;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Feature_Value_Entry {
    pub value_code: u8,
    pub value_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Feature_Metadata {
    pub marker: [c_char; 4],
    pub feature_code: DDCA_Vcp_Feature_Code,
    pub vcp_version: DDCA_MCCS_Version_Spec,
    pub feature_flags: DDCA_Feature_Flags,
    pub sl_values: *mut DDCA_Feature_Value_Entry,
    pub unused: *mut c_void,
    pub feature_name: *mut c_char,
    pub feature_desc: *mut c_char,
}

// Represent the Capabilities string returned by a monitor
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Cap_Vcp {
    pub marker: [c_char; 4],
    pub feature_code: DDCA_Vcp_Feature_Code,
    pub value_ct: c_int,
    pub values: *mut u8,
}

impl DDCA_Cap_Vcp {
    pub fn values(&self) -> &[u8] {
        unsafe { from_raw_parts(self.values as *const _, self.value_ct as usize) }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Capabilities {
    pub marker: [c_char; 4],
    pub unparsed_string: *mut c_char,
    pub version_spec: DDCA_MCCS_Version_Spec,
    pub cmd_ct: c_int,
    pub cmd_codes: *mut u8,
    pub vcp_code_ct: c_int,
    pub vcp_codes: *mut DDCA_Cap_Vcp,
    pub msg_ct: c_int,
    pub messages: *mut *mut c_char,
}

impl DDCA_Capabilities {
    pub fn vcp_codes(&self) -> &[DDCA_Cap_Vcp] {
        unsafe { from_raw_parts(self.vcp_codes as *const _, self.vcp_code_ct as usize) }
    }
}

// Get and set VCP feature values
pub type DDCA_Vcp_Value_Type = c_int;
pub const DDCA_NON_TABLE_VCP_VALUE: DDCA_Vcp_Value_Type = 1;
pub const DDCA_TABLE_VCP_VALUE: DDCA_Vcp_Value_Type = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Non_Table_Vcp_Value {
    pub mh: u8,
    pub ml: u8,
    pub sh: u8,
    pub sl: u8,
}

impl DDCA_Non_Table_Vcp_Value {
    pub fn value(&self) -> u16 {
        ((self.sh as u16) << 8) | self.sl as u16
    }

    pub fn maximum(&self) -> u16 {
        ((self.mh as u16) << 8) | self.ml as u16
    }
}

#[repr(C)]
pub struct DDCA_Table_Vcp_Value {
    pub bytect: u16,
    pub bytes: *mut u8,
}

impl DDCA_Table_Vcp_Value {
    pub fn bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self.bytes as *const _, self.bytect as usize) }
    }
}

impl fmt::Debug for DDCA_Table_Vcp_Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.bytes(), f)
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DDCA_Any_Vcp_Value {
    pub opcode: DDCA_Vcp_Feature_Code,
    pub value_type: DDCA_Vcp_Value_Type,
    pub val: DDCA_Any_Vcp_Value_Union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DDCA_Any_Vcp_Value_Union {
    pub t: DDCA_Table_Vcp_Value_Data,
    pub c_nc: DDCA_Non_Table_Vcp_Value,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_Table_Vcp_Value_Data {
    pub bytes: *mut u8,
    pub bytect: u16,
}

// For reporting display status changes to client
pub type DDCA_Display_Event_Type = c_int;
pub const DDCA_EVENT_DPMS_AWAKE: DDCA_Display_Event_Type = 0;
pub const DDCA_EVENT_DPMS_ASLEEP: DDCA_Display_Event_Type = 1;
pub const DDCA_EVENT_DISPLAY_CONNECTED: DDCA_Display_Event_Type = 2;
pub const DDCA_EVENT_DISPLAY_DISCONNECTED: DDCA_Display_Event_Type = 3;
pub const DDCA_EVENT_DDC_ENABLED: DDCA_Display_Event_Type = 4;
pub const DDCA_EVENT_UNUSED2: DDCA_Display_Event_Type = 5;

pub type DDCA_Display_Event_Class = c_int;
pub const DDCA_EVENT_CLASS_NONE: DDCA_Display_Event_Class = 0;
pub const DDCA_EVENT_CLASS_DPMS: DDCA_Display_Event_Class = 1;
pub const DDCA_EVENT_CLASS_DISPLAY_CONNECTION: DDCA_Display_Event_Class = 2;
pub const DDCA_EVENT_CLASS_UNUSED1: DDCA_Display_Event_Class = 4;
pub const DDCA_EVENT_CLASS_ALL: DDCA_Display_Event_Class =
    DDCA_EVENT_CLASS_DPMS | DDCA_EVENT_CLASS_DISPLAY_CONNECTION;

pub const DDCA_DISPLAY_EVENT_DDC_WORKING: u8 = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DDCA_Display_Status_Event {
    pub timestamp_nanos: u64,
    pub event_type: DDCA_Display_Event_Type,
    pub io_path: DDCA_IO_Path,
    pub connector_name: [c_char; 32],
    pub dref: DDCA_Display_Ref,
    pub flags: u8,
    pub unused: [*mut c_void; 1],
}

pub type DDCA_Display_Status_Callback_Func = extern "C" fn(event: DDCA_Display_Status_Event);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DDCA_DW_Settings {
    pub xevent_watch_interval_millisec: u16,
    pub poll_watch_interval_millisec: u16,
    pub initial_stabilization_millisec: u16,
    pub stabilization_poll_millisec: u16,
    pub watch_retry_interval_millisec: u16,
    pub unused: [*mut c_void; 4],
}
