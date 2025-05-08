buildscript {
    repositories {
        mavenLocal()
        maven {
            url "https://repo1.maven.org/maven2"
        }
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
        maven {
            url "https://repo1.maven.org/maven2"
        }
        google()
    }
}

tasks.register("clean").configure {
    delete("build")
}

