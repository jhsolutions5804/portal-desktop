"""tauri android init 이 생성한 app/build.gradle.kts 에 release 서명 설정을 주입한다.
Tauri CLI 자체는 Android APK 서명을 처리하지 않으므로, 여기서 keystore.properties
파일(같은 워크플로우에서 앞서 생성됨)을 읽어 signingConfig 를 붙여준다."""

path = "src-tauri/gen/android/app/build.gradle.kts"
content = open(path, encoding="utf-8").read()

old_import = "import java.util.Properties"
new_import = "import java.util.Properties\nimport java.io.FileInputStream"
assert content.count(old_import) == 1, "import anchor not found"
content = content.replace(old_import, new_import, 1)

old_props = (
    "val tauriProperties = Properties().apply {\n"
    "    val propFile = file(\"tauri.properties\")\n"
    "    if (propFile.exists()) {\n"
    "        propFile.inputStream().use { load(it) }\n"
    "    }\n"
    "}"
)
new_props = old_props + (
    "\n\nval keystoreProperties = Properties().apply {\n"
    "    val propFile = rootProject.file(\"keystore.properties\")\n"
    "    if (propFile.exists()) {\n"
    "        FileInputStream(propFile).use { load(it) }\n"
    "    }\n"
    "}"
)
assert content.count(old_props) == 1, "tauriProperties block not found"
content = content.replace(old_props, new_props)

old_android_open = "android {\n    compileSdk = 36"
new_android_open = (
    "android {\n"
    "    signingConfigs {\n"
    "        create(\"release\") {\n"
    "            storeFile = file(keystoreProperties.getProperty(\"storeFile\", \"none\"))\n"
    "            storePassword = keystoreProperties.getProperty(\"storePassword\", \"\")\n"
    "            keyAlias = keystoreProperties.getProperty(\"keyAlias\", \"\")\n"
    "            keyPassword = keystoreProperties.getProperty(\"keyPassword\", \"\")\n"
    "        }\n"
    "    }\n"
    "    compileSdk = 36"
)
assert content.count(old_android_open) == 1, "android block open not found"
content = content.replace(old_android_open, new_android_open)

old_release_block = "getByName(\"release\") {\n            isMinifyEnabled = true"
new_release_block = (
    "getByName(\"release\") {\n"
    "            signingConfig = signingConfigs.getByName(\"release\")\n"
    "            isMinifyEnabled = true"
)
assert content.count(old_release_block) == 1, "release buildType block not found"
content = content.replace(old_release_block, new_release_block)

open(path, "w", encoding="utf-8").write(content)
print("build.gradle.kts patched for signing")
