* 설치 항목
  * rustc
    * Rust 컴파일러
  * cargo
    * Rust 패키지 매니저
* 설치
  ``` base
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* 확인
  * rustc
    ```
    rustc --version
    ```

    ```
    # rustc 1.96.0 (ac68faa20 2026-05-25)
    ```
  * cargo
    ```
    cargo --version
    ```

    ```
    #cargo 1.96.0 (30a34c682 2026-05-25)
    ```

* 테스트
  * hello_world 프로젝트 생성 
    ```
    cd ~/Study/rust_study/playground 
    cargo new hello_world
    ```

    ```
    Creating binary (application) `hello_world` package
    note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    ```

  * 폴더 구조
    ```
    └── hello_world
        ├── Cargo.toml
        └── src
            └── main.rs
    ```

  * 실행
    ```
    cd ~/Study/rust_study/playground/hello_world
    cargo run
    ```

    ```
    Compiling hello_world v0.1.0 (/home/ldg/Study/rust_study/playground/hello_world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
    Running `target/debug/hello_world`
    
    Hello, world!
    ```
