const XTASK_RUNTIME_VAR = "xtask_runtime";

let xtask_runtime = variable::get(XTASK_RUNTIME_VAR);

if xtask_runtime == "tokio" {
    file::delete("xtask/main_smol.rs");
    file::delete("xtask/task_smol.rs");
    file::delete("xtask/main_sync.rs");
    file::delete("xtask/task_sync.rs");
    file::rename("xtask/main_tokio.rs", "xtask/main.rs");
    file::rename("xtask/task_tokio.rs", "xtask/task.rs");
} else if xtask_runtime == "smol" {
    file::delete("xtask/main_tokio.rs");
    file::delete("xtask/task_tokio.rs");
    file::delete("xtask/main_sync.rs");
    file::delete("xtask/task_sync.rs");
    file::rename("xtask/main_smol.rs", "xtask/main.rs");
    file::rename("xtask/task_smol.rs", "xtask/task.rs");
} else if xtask_runtime == "sync" {
    file::delete("xtask/main_smol.rs");
    file::delete("xtask/task_smol.rs");
    file::delete("xtask/main_tokio.rs");
    file::delete("xtask/task_tokio.rs");
    file::rename("xtask/main_sync.rs", "xtask/main.rs");
    file::rename("xtask/task_sync.rs", "xtask/task.rs");
}