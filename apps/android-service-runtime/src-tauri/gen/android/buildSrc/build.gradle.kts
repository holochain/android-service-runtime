plugins {
    `kotlin-dsl`
}

gradlePlugin {
    plugins {
        create("pluginsForCoolKids") {
            id = "rust"
            implementationClass = "RustPlugin"
        }
    }
}

repositories {
    mavenLocal()
    maven {
        url("https://repo1.maven.org/maven2")
    }
    google()
}

dependencies {
    compileOnly(gradleApi())
    implementation("com.android.tools.build:gradle:8.6.1")
}

