import org.jmailen.gradle.kotlinter.tasks.ConfigurableKtLintTask

plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
    id("org.jmailen.kotlinter") version("5.0.1")
}

// The uniffi-generated kotlin bindings violate some linter rules
tasks.withType<ConfigurableKtLintTask> {
    exclude("**/*_ffi.kt")
}

android {
    buildToolsVersion = "34.0.0"
    namespace = "org.holochain.androidserviceruntime.plugin.client"
    compileSdk = 34

    defaultConfig {
        minSdk = 27

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    buildFeatures {
        aidl = true
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
}

dependencies {
    // Subprojects
    implementation(project(":tauri-android"))
    implementation("org.holochain.androidserviceruntime:client:0.0.18")
    // Kotlin
    implementation("androidx.core:core-ktx:1.7.0")
    implementation("androidx.appcompat:appcompat:1.6.0")
    implementation("com.google.android.material:material:1.7.0")
    // Coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.10.1")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.10.1")
    // Uniffi
    implementation("net.java.dev.jna:jna:5.16.0@aar")
    implementation("org.jetbrains.kotlin:kotlin-reflect:2.1.10")
    // Tests
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
}
