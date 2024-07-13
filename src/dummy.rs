//! 外からモジュールをインポートする際はpubを付ける必要がある
pub mod file {
    use rand::random;
    use std::fmt::Display;
    use std::fmt::{self, Formatter};

    /// ファイル状態列挙体
    #[derive(Debug, PartialEq)]
    enum FileState {
        Open,
        Close,
    }
    /// FileStateにDisplayトレイトを実装し、独自形式でprintln!に出力できるようにする
    impl Display for FileState {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match *self {
                FileState::Open => write!(f, "OPEN"),
                FileState::Close => write!(f, "CLOSE"),
            }
        }
    }

    /// ダミーファイル構造体
    #[derive(Debug)]
    pub struct File {
        name: String,
        data: Vec<u8>,
        state: FileState,
    }
    impl File {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                data: Vec::new(),
                state: FileState::Close,
            }
        }
        pub fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
            let mut f = File::new(name);
            f.data = data.clone();
            f
        }
        pub fn name(&self) -> String {
            self.name.clone()
        }
        pub fn len(&self) -> usize {
            self.data.len()
        }

        /// 乱数によりオープンを失敗させる
        pub fn open(&mut self) -> Result<bool, String> {
            if random::<bool>() {
                self.state = FileState::Close;
                Err(String::from("file open error"))
            } else {
                self.state = FileState::Open;
                Ok(true)
            }
        }

        /// 乱数によりクローズを失敗させる
        pub fn close(&mut self) -> Result<bool, String> {
            if random::<bool>() {
                self.state = FileState::Open;
                Err(String::from("file close error"))
            } else {
                self.state = FileState::Close;
                Ok(true)
            }
        }
    }

    /// 外から呼ぶ場合はpubをつける必要がある
    pub trait Read {
        fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
    }
    impl Read for File {
        fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
            if self.state != FileState::Open {
                return Err(String::from("File must be open for reading"));
            }
            let mut tmp = self.data.clone();
            let read_length = tmp.len();
            save_to.reserve(read_length);
            save_to.append(&mut tmp);
            Ok(read_length)
        }
    }
    /// FileにDisplayトレイトを実装し、独自形式でprintln!に出力できるようにする
    impl Display for File {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "<{}{}>", self.name, self.state)
        }
    }
}
