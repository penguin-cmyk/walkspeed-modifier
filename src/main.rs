mod utils;
use utils::utils::Utils;
use memory_utils::process::Process;
fn main() {
    let roblox_prc = Process::new(Process::pid("RobloxPlayerBeta.exe").unwrap());
    let base_address = roblox_prc.get_base_address().unwrap() as usize;

    let rebase = |address: usize| -> usize { base_address + address };
    let fake_dm_pointer = rebase(0x66EA5E8);

    let fake_dm = roblox_prc.read_memory::<usize>(fake_dm_pointer).unwrap();
    if fake_dm == 0 {
        println!("Failed to get the fake datamodel");
        return
    }

    let real_dm = roblox_prc.read_memory::<usize>(fake_dm + 0x1B8).unwrap();

    let utils = Utils::new(&roblox_prc);
    let players = utils.find_first_child(real_dm, "Players".to_string());

    let localplayer = roblox_prc.read_memory::<usize>(players + 0x128).unwrap();
    let character = roblox_prc.read_memory::<usize>(localplayer + 0x330).unwrap();
    let humanoid = utils.find_first_child(character, "Humanoid".to_string());

    roblox_prc.write_memory::<f32>(humanoid + 0x1D8, &200.0).unwrap(); // WalkSpeed
    roblox_prc.write_memory::<f32>(humanoid + 0x3B0, &200.0).unwrap(); // WalkSpeedCheck
}
