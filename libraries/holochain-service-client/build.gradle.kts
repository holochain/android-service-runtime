import com.vanniktech.maven.publish.SonatypeHost

plugins {
  id("com.android.library")
  id("org.jetbrains.kotlin.android")
  id("org.jetbrains.kotlin.plugin.parcelize")
  id("com.vanniktech.maven.publish") version ("0.30.0")
  id("org.jmailen.kotlinter") version("5.0.1")
}

// The uniffi-generated kotlin bindings violate some of ktlint's rules.
// Thus, we still allow builds with format and lint violations.
kotlinter {
    ignoreFormatFailures = true
    ignoreLintFailures = true
}

android {
  namespace = "org.holochain.androidserviceruntime.holochain_service_client"
  compileSdk = 34

  defaultConfig {
    minSdk = 27

    testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    consumerProguardFiles("consumer-rules.pro")
  }

  buildTypes {
    release {
      isMinifyEnabled = false
      proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
    }
  }
  buildFeatures { aidl = true }
  compileOptions {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
  }
  kotlinOptions { jvmTarget = "1.8" }
}

mavenPublishing {
  publishToMavenCentral(SonatypeHost.CENTRAL_PORTAL)

  signAllPublications()

  coordinates("org.holochain.androidserviceruntime", "holochain-service-client", "0.0.12")

  pom {
    name.set("Holochain Service Client")
    description.set(
        "Client for binding and making calls to the HolochainService provided by android-service-runtime app.")
    inceptionYear.set("2025")
    url.set("https://github.com/holochain/android-service-runtime/")
    licenses {
      license {
        name.set("The Cryptographic Autonomy License version 1.0")
        url.set("https://opensource.org/license/cal-1-0")
        distribution.set("https://opensource.org/license/cal-1-0")
      }
    }
    developers {
      developer {
        id.set("mattyg")
        name.set("mattyg")
        url.set("https://github.com/mattyg")
      }
    }
    scm {
      url.set("https://github.com/holochain/android-service-runtime/")
      connection.set("scm:git:git://github.com/holochain/android-service-runtime.git")
      developerConnection.set("scm:git:ssh://git@github.com/holochain/android-service-runtime.git")
    }
  }
}

dependencies {
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
