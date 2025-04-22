use std::process::Command;

fn main() {
    let ps_script = r#"
    Get-NetTCPConnection -State Listen | ForEach-Object {
        try {
            $p = Get-Process -Id $_.OwningProcess
            "$($_.LocalAddress):$($_.LocalPort) -> $($p.Name) ($($p.Id))"
        } catch {
            "$($_.LocalAddress):$($_.LocalPort) -> PID $($_.OwningProcess) (acesso negado)"
        }
    }
    "#;

    let output = Command::new("powershell")
        .args(&["-Command", ps_script])
        .output()
        .expect("Falha ao executar PowerShell");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
