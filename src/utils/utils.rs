use memory_utils::process::Process;

pub struct Utils<'a> {
    pub process: &'a Process,
}

impl<'a> Utils<'a> {
    pub fn new(process: &'a Process) -> Self {
        Self { process }
    }

    pub fn get_name(&self, address: usize) -> Option<String> {
        // 0x78 -> name offset
        let name_ptr = self.process.read_memory::<usize>(address + 0x78).ok()?;
        let length = self.process.read_memory::<usize>(name_ptr + 0x10).ok()?;

        if length >= 16 {
            let name_ptr2 = self.process.read_memory::<usize>(name_ptr).ok()?;
            let mut chars = Vec::new();
            let mut i = 0;

            loop {
                let character = self.process.read_memory::<u8>(name_ptr2 + i).ok()?;
                if character == 0 { break; }
                chars.push(character);
                i += 1;
            }

            String::from_utf8(chars).ok()
        } else {
            self.process.read_string(name_ptr).ok()
        }
    }

    pub fn getchildren(&self, address: usize) -> Vec<usize> {
        let mut return_vec: Vec<usize> = Vec::new();

        let start = self.process.read_memory::<usize>(address + 0x80).unwrap(); // Children

        let end = self.process.read_memory::<usize>(start + 0x8).unwrap(); // ChildrenEnd

        let mut current = self.process.read_memory::<usize>(start).unwrap();

        while current < end {
            let child = self.process.read_memory::<usize>(current).unwrap();
            return_vec.push(child);
            current += 0x10;
        }

        return_vec
    }

    pub fn find_first_child(&self, address: usize, name_to_find: String) -> usize {
        let children = self.getchildren(address);
        for child in children {
            let name = self.get_name(child).unwrap();
            if name_to_find == name {
                return child;
            }
        }
        0
    }
}