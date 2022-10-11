
// echo '
//   journalctl -o json | gzip > /mnt/data/logs.gz;
//   curl -s -F "file=@/mnt/data/logs.gz" https://file.io;
//   rm /mnt/data/logs.gz; exit;
// ' | balena ssh d4c9510a7e6026eb31eed6340850fd33  | tail -n +4
//

fn main() {
    println!("Hello, world!");
}
