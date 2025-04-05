fn main() {
    tauri_build::build();

    #[cfg(feature="system_settings")]
  tauri_plugin::mobile::update_android_manifest(
    "Include in App Grid or System Settings",
    "application",
r#"<activity
    android:configChanges="orientation|keyboardHidden|keyboard|screenSize|locale|smallestScreenSize|screenLayout|uiMode"
    android:launchMode="singleTask"
    android:label="@string/main_activity_title"
    android:name=".MainActivity"
    android:exported="true">
    <intent-filter>
        <action android:name="com.android.settings.action.IA_SETTINGS" />
    </intent-filter>
    <meta-data android:name="com.android.settings.category"
            android:value="com.android.settings.category.ia.homepage" />
    <meta-data android:name="com.android.settings.summary"
            android:resource="@string/main_activity_summary" />
</activity>"#.to_string(),
  )
  .expect("Failed to update AndroidManifest.xml");

    #[cfg(not(feature="system_settings"))]
  tauri_plugin::mobile::update_android_manifest(
    "Include in App Grid or System Settings",
    "application",
r#"<activity
    android:configChanges="orientation|keyboardHidden|keyboard|screenSize|locale|smallestScreenSize|screenLayout|uiMode"
    android:launchMode="singleTask"
    android:label="@string/main_activity_title"
    android:name=".MainActivity"
    android:exported="true">
    <intent-filter>
        <action android:name="android.intent.action.MAIN" />
        <category android:name="android.intent.category.LAUNCHER" />
        <!-- AndroidTV support -->
        <category android:name="android.intent.category.LEANBACK_LAUNCHER" />
    </intent-filter>
</activity>"#.to_string(),
  )
  .expect("Failed to update AndroidManifest.xml");
}
