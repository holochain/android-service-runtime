buildscript {
    repositories {
        mavenLocal()
        mavenCentral()
        google()
    }
    dependencies {
        classpath("com.android.tools.build:gradle:8.6.1")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:2.1.10")
    }
}

allprojects {
    repositories {
        mavenLocal()
        mavenCentral()
        google()
    }
}

tasks.register("clean").configure {
    delete("build")
}

