# Copy Without Folder

Công cụ dòng lệnh đơn giản để sao chép file từ thư mục nguồn sang thư mục đích với khả năng loại trừ các thư mục không mong muốn. Chương trình sẽ hướng dẫn bạn từng bước một cách tương tác.

## Cài đặt

```bash
# Clone repository  
git clone <repository-url>
cd copy-without-folder

# Build project
cargo build --release
```

## Sử dụng

### Chạy chương trình

```bash
cargo run
```

Chương trình sẽ yêu cầu bạn nhập:

1. **Thư mục nguồn**: Đường dẫn đến thư mục chứa file cần sao chép
2. **Thư mục đích**: Đường dẫn đến nơi muốn lưu file đã sao chép  
3. **Thư mục loại trừ**: Danh sách các thư mục không muốn sao chép (phân cách bằng dấu phẩy)

### Ví dụ sử dụng thực tế

#### Khi chạy chương trình:

```
Nhập folder source: /home/user/my-project
Nhập folder destination: /home/user/backup
Nhập tên folder cần loại trừ (phân cách bởi dấu ,): node_modules, .git, target
🔍 Đang scan...
📦 Tổng số file sẽ copy: 1250
🚫 Exclude folders: ["node_modules", ".git", "target"]
⚠️ Total excluded: 15000
Tiến hành copy? (y/n): y
```

Sau đó chương trình sẽ hiển thị progress bar và tiến hành sao chép.

## Tính năng

- ✅ Giao diện tương tác thân thiện với người dùng
- ✅ Scan và đếm số lượng file trước khi sao chép
- ✅ Sao chép tất cả file từ thư mục nguồn
- ✅ Tạo cấu trúc thư mục tự động ở đích
- ✅ Loại trừ nhiều thư mục cùng lúc (phân cách bằng dấu phẩy)
- ✅ Progress bar hiển thị tiến độ với emoji và thống kê chi tiết
- ✅ Xử lý lỗi và thông báo chi tiết
- ✅ Xác nhận trước khi tiến hành copy

## Cách hoạt động

1. **Scan giai đoạn 1**: Chương trình sẽ duyệt qua toàn bộ thư mục nguồn để:
   - Đếm tổng số file sẽ được copy
   - Xác định số lượng file/thư mục bị loại trừ
   - Hiển thị thống kê chi tiết

2. **Xác nhận**: Yêu cầu người dùng xác nhận trước khi thực hiện copy

3. **Thực hiện copy**: Sao chép từng file với progress bar hiển thị tiến độ

## Loại trừ thư mục

Chương trình sẽ loại trừ:
- Toàn bộ thư mục có tên khớp với danh sách loại trừ
- Tất cả file và thư mục con bên trong các thư mục bị loại trừ
- Tìm kiếm theo tên chính xác (case-sensitive)

### Ví dụ về loại trừ

Nếu bạn nhập: `node_modules, .git, target, build`

Chương trình sẽ bỏ qua:
- `./node_modules/` và tất cả nội dung bên trong
- `./frontend/node_modules/` và tất cả nội dung bên trong  
- `./.git/` và tất cả nội dung bên trong
- `./rust-project/target/` và tất cả nội dung bên trong
- `./cpp-project/build/` và tất cả nội dung bên trong

## Lưu ý quan trọng

1. **Thư mục nguồn phải tồn tại**: Chương trình sẽ báo lỗi nếu không tìm thấy thư mục nguồn.

2. **Thư mục đích được tạo tự động**: Nếu thư mục đích chưa tồn tại, chương trình sẽ tạo tự động khi cần thiết.

3. **Loại trừ theo tên chính xác**: Chương trình sẽ loại trừ tất cả thư mục có tên khớp chính xác với danh sách đã nhập.

4. **Ghi đè file**: File đã tồn tại ở thư mục đích sẽ bị ghi đè không cần hỏi.

5. **Hiệu suất**: Chương trình scan 2 lần - lần đầu để đếm, lần hai để copy. Điều này đảm bảo progress bar chính xác.

## Các trường hợp sử dụng phổ biến

### Backup dự án web (loại trừ node_modules)
```
Nhập folder source: ./my-web-project
Nhập folder destination: ./backup/web-project-backup
Nhập tên folder cần loại trừ: node_modules, .git, dist, build
```

### Backup dự án Rust (loại trừ target)
```
Nhập folder source: ./my-rust-project  
Nhập folder destination: ./backup/rust-project-backup
Nhập tên folder cần loại trừ: target, .git
```

### Backup dự án đa ngôn ngữ
```
Nhập folder source: ./monorepo-project
Nhập folder destination: ./backup/monorepo-backup
Nhập tên folder cần loại trừ: node_modules, target, build, dist, .git, __pycache__, venv
```

### Copy thư mục media (không loại trừ gì)
```
Nhập folder source: ./photos-2024
Nhập folder destination: ./backup/photos-2024
Nhập tên folder cần loại trừ: (để trống, nhấn Enter)
```

## Build và chạy executable

### Build bản release
```bash
cargo build --release
```

### Chạy executable trực tiếp
```bash
./target/release/copy-without-folder
```

### Tạo alias để sử dụng dễ dàng (tùy chọn)

Thêm vào `~/.bashrc` hoặc `~/.zshrc`:
```bash
alias copy-clean="/path/to/your/project/target/release/copy-without-folder"
```

Sau đó bạn có thể chạy:
```bash
copy-clean
```

## Thông tin kỹ thuật

- **Ngôn ngữ**: Rust
- **Dependencies chính**:
  - `walkdir`: Duyệt thư mục đệ quy
  - `indicatif`: Progress bar và spinner
- **Platform**: Cross-platform (Windows, macOS, Linux)
- **Hiệu suất**: Tối ưu cho việc sao chép file lớn với progress tracking

## Troubleshooting

### Lỗi permission denied
```
❌ Không thể copy /path/to/file: Permission denied (os error 13)
```
**Giải pháp**: Kiểm tra quyền đọc thư mục nguồn và quyền ghi thư mục đích.

### Thư mục nguồn không tồn tại
Chương trình sẽ báo lỗi ngay khi bắt đầu scan. Kiểm tra lại đường dẫn đã nhập.

### Hết dung lượng đĩa
```
❌ Không thể copy /path/to/file: No space left on device (os error 28)
```
**Giải pháp**: Giải phóng dung lượng đĩa hoặc chọn thư mục đích khác.

## Đóng góp

1. Fork project này
2. Tạo feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Mở Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.
