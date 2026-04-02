// ============================================================
// Merkle Tree Implementation
//
// Tambahkan di Cargo.toml:
// [dependencies]
// sha2 = "0.10"
// hex  = "0.4"
//
// sha2 : implementasi algoritma SHA-256
// hex  : konversi bytes → string hexadecimal (untuk display)
// ============================================================

use sha2::{Digest, Sha256};

// ============================================================
// NODE — unit terkecil dari tree
//
// Enum adalah tipe yang bisa jadi salah satu dari beberapa
// "bentuk" yang didefinisikan. Di sini Node bisa jadi:
//   - Leaf   : node paling bawah, menyimpan data asli
//   - Branch : node tengah/atas, punya dua anak
//
// Ini disebut "recursive type" karena Branch menyimpan
// Node di dalamnya — tapi harus dibungkus Box<>.
//
// Box<T> — pointer ke heap
//   Rust perlu tahu ukuran tiap tipe saat compile time.
//   Node yang punya field Node di dalamnya ukurannya tidak
//   terbatas (infinite size). Box memecahkan ini: ukuran Box
//   selalu tetap (pointer = 8 byte), isinya di heap.
// ============================================================
#[derive(Debug)]
enum Node {
    Leaf {
        hash: String,
        data: String,
    },
    Branch {
        hash: String,
        left: Box<Node>, // Box = pointer ke heap, ukuran tetap
        right: Box<Node>,
    },
}

impl Node {
    // Method untuk ambil hash dari node apapun (Leaf atau Branch)
    // &self = pinjam node, tidak ambil ownership
    fn hash(&self) -> &str {
        match self {
            Node::Leaf { hash, .. } => hash,
            // ".." artinya "abaikan field lain yang tidak disebutkan"
            Node::Branch { hash, .. } => hash,
        }
    }
}

// ============================================================
// HASH FUNCTIONS
// ============================================================

// Hash satu string — untuk data di Leaf
fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    // update() memberi data ke hasher
    hasher.update(data.as_bytes());
    // finalize() selesaikan perhitungan, return array bytes
    let result = hasher.finalize();
    // hex::encode ubah bytes → string hex seperti "a3f2..."
    hex::encode(result)
}

// Hash dua hash digabung — untuk node Branch
fn hash_pair(left: &str, right: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(left.as_bytes());
    hasher.update(right.as_bytes());
    hex::encode(hasher.finalize())
}

// ============================================================
// BUILD TREE
// Membangun Merkle tree dari list data (Vec<String>)
// Return: Node root dari tree yang sudah jadi
// ============================================================
fn build_tree(data: &[String]) -> Node {
    // Base case — tidak boleh data kosong
    assert!(!data.is_empty(), "Data tidak boleh kosong");

    // Buat semua Leaf node dulu dari data mentah
    // iter().map().collect() = transformasi setiap elemen Vec
    let mut nodes: Vec<Node> = data
        .iter()
        .map(|d| Node::Leaf {
            hash: hash_data(d),
            data: d.clone(),
        })
        .collect();

    // Kalau hanya satu data, langsung return Leaf-nya
    if nodes.len() == 1 {
        return nodes.remove(0);
    }

    // Bangun tree dari bawah ke atas
    // Terus gabungkan pasang-pasang node sampai tersisa satu (root)
    while nodes.len() > 1 {
        let mut next_level: Vec<Node> = Vec::new();

        // Proses dua node sekaligus per iterasi
        // chunks(2) memecah Vec jadi potongan berisi 2 elemen
        // Kalau jumlah node ganjil, potongan terakhir berisi 1
        let mut i = 0;
        while i < nodes.len() {
            if i + 1 < nodes.len() {
                // Ada pasangan — buat Branch dari dua node
                // nodes.remove(i) ambil node dan hapus dari Vec
                // Harus remove dua kali: setelah remove(i),
                // elemen berikutnya bergeser ke index i juga
                let left = nodes.remove(i);
                let right = nodes.remove(i);

                let branch_hash = hash_pair(left.hash(), right.hash());

                next_level.push(Node::Branch {
                    hash: branch_hash,
                    left: Box::new(left), // Box::new() pindahkan ke heap
                    right: Box::new(right),
                });
            } else {
                // Node ganjil di akhir — angkat langsung ke level atas
                // tanpa dipasangkan (konvensi umum Merkle tree)
                let lone = nodes.remove(i);
                next_level.push(lone);
            }
        }

        // Ganti nodes dengan hasil level ini
        // Ulangi sampai tersisa satu node (root)
        nodes = next_level;
    }

    // nodes sekarang berisi tepat satu elemen: root
    nodes.remove(0)
}

// ============================================================
// PRINT TREE — visualisasi struktur tree ke terminal
//
// prefix  : string indentasi untuk baris ini
// is_left : apakah node ini anak kiri (untuk karakter grafis)
// ============================================================
fn print_tree(node: &Node, prefix: &str, is_left: bool) {
    // Tentukan karakter grafis berdasarkan posisi node
    let connector = if is_left { "├── " } else { "└── " };

    match node {
        Node::Leaf { hash, data } => {
            println!("{}{}[Leaf] data: {:?}", prefix, connector, data);
            println!("{}{}       hash: {}...", prefix, "    ", &hash[..12]);
        }
        Node::Branch { hash, left, right } => {
            println!("{}{}[Branch] hash: {}...", prefix, connector, &hash[..12]);

            // Buat prefix baru untuk anak-anak node ini
            // String concatenation dengan format!
            let new_prefix = format!("{}{}   ", prefix, if is_left { "│" } else { " " });

            print_tree(left, &new_prefix, true); // rekursi ke kiri
            print_tree(right, &new_prefix, false); // rekursi ke kanan
        }
    }
}

// ============================================================
// MERKLE PROOF
// Buktikan bahwa satu data ada di tree tanpa cek semua data.
// Return: Vec berisi hash-hash yang dibutuhkan untuk verifikasi
// ============================================================
fn generate_proof(node: &Node, target: &str, proof: &mut Vec<String>) -> bool {
    match node {
        Node::Leaf { data, .. } => {
            // Cek apakah ini data yang dicari
            data == target
        }
        Node::Branch { left, right, .. } => {
            // Coba cari di kiri dulu
            if generate_proof(left, target, proof) {
                // Target ada di kiri → simpan hash kanan sebagai bukti
                proof.push(right.hash().to_string());
                true
            } else if generate_proof(right, target, proof) {
                // Target ada di kanan → simpan hash kiri sebagai bukti
                proof.push(left.hash().to_string());
                true
            } else {
                false
            }
        }
    }
}

// Verifikasi proof: recompute root dari data + proof, bandingkan
fn verify_proof(data: &str, proof: &[String], expected_root: &str) -> bool {
    // Mulai dari hash data yang mau diverifikasi
    let mut current_hash = hash_data(data);

    // Kombinasikan dengan tiap hash di proof secara berurutan
    for proof_hash in proof {
        current_hash = hash_pair(&current_hash, proof_hash);
    }

    // Hash akhir harus sama dengan Merkle Root
    current_hash == expected_root
}

// ============================================================
// MAIN
// ============================================================
fn main() {
    println!("=== Merkle Tree ===\n");

    // Data simulasi — bisa diganti transaksi, nama, apapun
    let data = vec![
        "TX: Andaru → Budi 50 XLM".to_string(),
        "TX: Budi → Citra 30 XLM".to_string(),
        "TX: Citra → Dani 20 XLM".to_string(),
        "TX: Dani → Andaru 10 XLM".to_string(),
    ];

    println!("Data input:");
    for (i, d) in data.iter().enumerate() {
        println!("  [{}] {}", i, d);
    }
    println!();

    // Build tree
    let tree = build_tree(&data);

    // Tampilkan Merkle Root
    println!("Merkle Root: {}\n", tree.hash());

    // Visualisasi struktur tree
    println!("Struktur tree:");
    println!("└── [Root]");
    if let Node::Branch { left, right, .. } = &tree {
        print_tree(left, "    ", true);
        print_tree(right, "    ", false);
    }
    println!();

    // Demo Merkle Proof
    let target = "TX: Budi → Citra 30 XLM";
    let mut proof = Vec::new();

    println!("--- Merkle Proof ---");
    println!("Membuktikan bahwa '{}' ada di tree...\n", target);

    if generate_proof(&tree, target, &mut proof) {
        println!("Proof berhasil dibuat:");
        for (i, h) in proof.iter().enumerate() {
            println!("  [{}] {}...", i, &h[..12]);
        }

        let valid = verify_proof(target, &proof, tree.hash());
        println!(
            "\nVerifikasi: {}",
            if valid {
                "✓ VALID"
            } else {
                "✗ TIDAK VALID"
            }
        );
    } else {
        println!("Data tidak ditemukan di tree.");
    }

    // Demo: kalau data dimanipulasi, root berubah
    println!("\n--- Demo Manipulasi ---");
    let data_manipulasi = vec![
        "TX: Andaru → Budi 50 XLM".to_string(),
        "TX: Budi → Citra 999 XLM".to_string(), // ← diubah!
        "TX: Citra → Dani 20 XLM".to_string(),
        "TX: Dani → Andaru 10 XLM".to_string(),
    ];
    let tree_manipulasi = build_tree(&data_manipulasi);
    println!("Root asli        : {}...", &tree.hash()[..20]);
    println!("Root dimanipulasi: {}...", &tree_manipulasi.hash()[..20]);
    println!(
        "Root sama? {}",
        if tree.hash() == tree_manipulasi.hash() {
            "Ya (tidak terdeteksi — bug!)"
        } else {
            "Tidak → manipulasi terdeteksi ✓"
        }
    );
}
