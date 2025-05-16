import com.vanniktech.maven.publish.SonatypeHost
import org.jmailen.gradle.kotlinter.tasks.ConfigurableKtLintTask

plugins {
  id("com.android.library")
  id("org.jetbrains.kotlin.android")
  id("com.vanniktech.maven.publish") version ("0.30.0")
  id("org.jmailen.kotlinter") version("5.0.1")
  id("org.jetbrains.dokka") version("2.0.0")
}

// The uniffi-generated kotlin bindings violate some linter rules
tasks.withType<ConfigurableKtLintTask> {
    exclude("**/*_ffi.kt")
}

android {
  buildToolsVersion = "34.0.0"
  namespace = "org.holochain.androidserviceruntime.service"
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

  if (!project.gradle.startParameter.taskNames.any { it.contains("publishToMavenLocal") }) {
      signAllPublications()
  }

  coordinates("org.holochain.androidserviceruntime", "service", "0.0.14")

  pom {
    name.set("Holochain Service")
    description.set("An Android Foreground Service that runs a Holochain conductor.")
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
  // Subprojects
  implementation("org.holochain.androidserviceruntime:client:0.0.14")
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
  androidTestImplementation("androidx.test.ext:junit:1.2.1")
  androidTestImplementation("androidx.test:core:1.6.1")
  androidTestImplementation("androidx.test:runner:1.6.2")
  androidTestImplementation("androidx.test:rules:1.6.1")
}
