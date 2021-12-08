# 5.3 async/await

각 디렉터리에 Cargo용 저장소가 있으므로, 디렉터리로 이동한 뒤 `cargo`로 컴파일 및 실행합니다.

빌드 시 ```--release```를 설정하면 올바른 효과를 알 수 있습니다.

## 컴파일과 실행 예시

다음과 같이 디렉터리로 이동한 뒤 실행합니다. `epoll`을 이용하므로 Linux만 대상이 됩니다.

```sh
$ cd ch5_3_2_ioselect
$ cargo run --release
```
