# GitHub Pages デプロイ設定

このリポジトリは自動的にGitHub Pagesにデプロイされます。

## セットアップ手順

1. リポジトリをpublicにする（すでにprivateの場合）
   - Settings → Danger Zone → Change repository visibility → Make public

2. GitHub Pagesを有効化
   - Settings → Pages
   - Source: **GitHub Actions** を選択
   - Save

3. ワークフローを実行
   - mainブランチにpushするか、Actions → Deploy to GitHub Pages → Run workflow

## アクセスURL

デプロイ後、以下のURLでアクセス可能になります：

- **リソースパック**: `https://[username].github.io/OfroPack/OfroPack.zip`
- **SHA256ハッシュ**: `https://[username].github.io/OfroPack/hash.txt`
- **メタデータ**: `https://[username].github.io/OfroPack/metadata.json`
- **Webページ**: `https://[username].github.io/OfroPack/`

## Minecraftサーバー設定

`server.properties`に以下を追加：

```properties
resource-pack=https://[username].github.io/OfroPack/OfroPack.zip
resource-pack-prompt=Ofro鯖のリソースパックをダウンロードしますか？
require-resource-pack=false
```

## プログラムからのアクセス例

### Bash/Shell

```bash
# ダウンロード
wget https://[username].github.io/OfroPack/OfroPack.zip

# ハッシュ検証
HASH=$(curl -s https://[username].github.io/OfroPack/hash.txt)
echo "Expected SHA256: $HASH"
sha256sum OfroPack.zip
```

### Python

```python
import requests
import hashlib

# ダウンロード
url = "https://[username].github.io/OfroPack/OfroPack.zip"
response = requests.get(url)

# ハッシュ検証
expected_hash = requests.get(url.replace('OfroPack.zip', 'hash.txt')).text.strip()
actual_hash = hashlib.sha256(response.content).hexdigest()

if expected_hash == actual_hash:
    with open('OfroPack.zip', 'wb') as f:
        f.write(response.content)
    print("Download successful and verified!")
else:
    print("Hash mismatch!")
```

### Java (Minecraft Plugin)

```java
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.security.MessageDigest;

public class ResourcePackDownloader {
    private static final String PACK_URL = "https://[username].github.io/OfroPack/OfroPack.zip";
    private static final String HASH_URL = "https://[username].github.io/OfroPack/hash.txt";
    
    public static void downloadAndVerify() throws Exception {
        // ダウンロード
        byte[] packData = new URL(PACK_URL).openStream().readAllBytes();
        
        // ハッシュ取得
        String expectedHash = new String(new URL(HASH_URL).openStream().readAllBytes()).trim();
        
        // 検証
        MessageDigest digest = MessageDigest.getInstance("SHA-256");
        String actualHash = bytesToHex(digest.digest(packData));
        
        if (expectedHash.equals(actualHash)) {
            Files.write(Paths.get("OfroPack.zip"), packData);
            System.out.println("Download verified!");
        } else {
            throw new SecurityException("Hash mismatch!");
        }
    }
    
    private static String bytesToHex(byte[] bytes) {
        StringBuilder result = new StringBuilder();
        for (byte b : bytes) {
            result.append(String.format("%02x", b));
        }
        return result.toString();
    }
}
```

## 注意事項

- デプロイには数分かかる場合があります
- キャッシュにより即座に反映されない場合があります（通常1-2分）
- GitHub Pagesは無料プランで月100GB帯域制限があります（通常は十分）
