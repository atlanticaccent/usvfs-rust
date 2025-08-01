use frida_gum::{Gum, Module};
use shared_types::HookError;

#[cfg(enable_for_ide)]
/// Module(s) here contain hooks the original usvfs added but don't seem to be necessary, either because of design
/// differences or potentially them not being necessary to hook in the first place regardless of implementation.
mod unused;

mod create_delete_directory;
mod file_attributes;
mod get_full_path_name;
mod private_profile_strings;

mod file_edit;
mod nt_close;
mod nt_create_file;
mod nt_open_file;
mod nt_query_directory_file;
mod nt_query_information_by_name;

pub(crate) use create_delete_directory::*;
pub(crate) use file_attributes::*;
pub(crate) use file_edit::*;
pub(crate) use nt_close::*;
pub(crate) use nt_create_file::*;
pub(crate) use nt_open_file::*;
pub(crate) use nt_query_directory_file::*;
pub(crate) use nt_query_information_by_name::*;
pub(crate) use private_profile_strings::*;

pub(crate) type FuncPatcher = fn(&Gum, &Module, &str) -> Result<(), HookError>;

pub static WIN32_TARGETS: [(&str, Option<FuncPatcher>); 34] = [
  ("GetFileAttributesExA", Some(get_file_attributes_ex_a)),
  ("GetFileAttributesA", Some(get_file_attributes_a)),
  ("GetFileAttributesExW", Some(get_file_attributes_ex_w)),
  ("GetFileAttributesW", Some(get_file_attributes_w)),
  ("SetFileAttributesW", Some(set_file_attributes_w)),
  ("CreateDirectoryW", Some(create_directory_w)),
  ("RemoveDirectoryW", Some(remove_directory_w)),
  ("GetCurrentDirectoryA", None),
  ("GetCurrentDirectoryW", None), // used in at least find_first_file
  ("SetCurrentDirectoryA", None),
  ("SetCurrentDirectoryW", None),
  ("CreateProcessInternalW", None),
  ("ExitProcess", None),
  ("DeleteFileW", Some(delete_file_w)),
  ("MoveFileA", Some(move_file_a)),
  ("MoveFileExA", Some(move_file_ex_a)),
  ("MoveFileW", Some(move_file_w)),
  ("MoveFileExW", Some(move_file_ex_w)),
  ("MoveFileWithProgressA", Some(move_file_with_progress_a)),
  ("MoveFileWithProgressW", Some(move_file_with_progress_w)),
  ("CopyFileExW", Some(copy_file_ex_w)),
  ("GetFullPathNameA", None),
  ("GetFullPathNameW", None),
  ("LoadLibraryExA", None),
  ("LoadLibraryExW", None),
  ("GetModuleFileNameA", None),
  ("GetModuleFileNameW", None),
  // These should only be necessary for 16-bit application compatibility
  (
    "GetPrivateProfileStringA",
    Some(get_private_profile_string_a),
  ),
  (
    "GetPrivateProfileStringW",
    Some(get_private_profile_string_w),
  ),
  ("GetPrivateProfileSectionA", None),
  ("GetPrivateProfileSectionW", None),
  ("WritePrivateProfileStringA", None),
  ("WritePrivateProfileStringW", None),
  // Unnecessary
  ("FindFirstFileExW", None),
];

pub static WIN8_PLUS_WIN32_TARGETS: [(&str, Option<FuncPatcher>); 1] = [("CopyFile2", None)];

pub static NT_TARGETS: [(&str, Option<FuncPatcher>); 11] = [
  ("NtQueryDirectoryFile", Some(nt_query_directory_file)),
  ("NtQueryDirectoryFileEx", Some(nt_query_directory_file_ex)),
  (
    "NtQueryInformationByName",
    Some(nt_query_information_by_name),
  ),
  ("NtOpenFile", Some(nt_open_file)),
  ("NtCreateFile", Some(nt_create_file)),
  ("NtClose", Some(nt_close)),
  ("NtTerminateProcess", None),
  // Unnecessary
  ("NtQueryObject", None),
  ("NtQueryInformationFile", None),
  ("NtQueryFullAttributesFile", None),
  ("NtQueryAttributesFile", None),
];
