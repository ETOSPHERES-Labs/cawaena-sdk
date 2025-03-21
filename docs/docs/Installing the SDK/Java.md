# Java Installation

The Java SDK is now available through a private Maven repository hosted on JFrog. You can add it as a dependency in your Maven project's `pom.xml` file.

## Installing via Maven Repository

1. Update `settings.xml`

    To access the private Maven repository, you need to update your Maven `settings.xml` file, usually located in the `~/.m2` directory. Add the following server configuration:

    ```xml
    <servers>
    <server>
        <id>jfrog-private-repo</id>
        <username>your-username</username>
        <password>your-password</password>
        </server>
    </servers>
    ```

    > Note: Replace `your-username` and `your-password` with your actual JFrog repository credentials.

2. Update your projects `pom.xml` file to include the `dependency` and the `repository`:

    ```xml
    <dependency>
        <groupId>com.etogruppe</groupId>
        <artifactId>ETOPaySdk</artifactId>
        <version>0.11.0</version>
    </dependency>
    ```

    ```xml
    <repositories>
    <repository>
        <id>snapshots-repo</id>
        <url>https://repo.farmunited.com:443/artifactory/egdbz-mvn/</url>
        <releases>
        <enabled>false</enabled>
        </releases>
        <snapshots>
        <enabled>true</enabled>
        </snapshots>
    </repository>
    </repositories>
    ```

See [JFrog Artifactory Documentation > Package Management > Maven Repository](https://jfrog.com/help/r/jfrog-artifactory-documentation/maven-repository) for guiding docs.

## Installing jniLibs

The jar files also contain the jniLibs folder. The jniLibs folder in turn contains the pre-built shared object libraries of the SDK. The structure of the folder is as shown below:

```
jniLibs
├── arm64-v8a
│   ├── libetopaysdk.so
│   └── libc++_shared.so
├── armeabi-v7a
│   ├── libetopaysdk.so
│   └── libc++_shared.so
├── x86
│   ├── libetopaysdk.so
│   └── libc++_shared.so
└── x86_64
    ├── libetopaysdk.so
    └── libc++_shared.so
```

The `jniLibs` folder should be placed **as-is** under the `src/main` folder of the corresponding Java project from the jar file.

!!! warning
    Currently the moving of the jniLibs from the jar to the src/main folder is manual and not automated. Later it will be automated with a gradle plugin once the project is moved to open source. Not copying this generally will throw the error while initializing the constructor of the ETOPaySdk class `java.lang.UnsatisfiedLinkError: dlopen failed: library "libetopaysdk.so" not found`.

## Future releases of SDK

Future releases of the SDK will continue to be published to the private Maven repository. You will only need to update the dependency version in your pom.xml file to use the latest release and replace the jniLibs correspondingly.

## Minimum version support

The following versions of the toolchain are used to build and compile the Java SDK and should be used as minimum versions for integrating the SDK. Versions lower than the mentioned might work, however are not guaranteed by the team. In case of issues, please contact the team with specific build or compile errors.

- **Java Compiler**: `17.0.10`
- **gradle**: `Gradle 8.6`
- **Android SDK Command-line Tools**:  `12.0`
- **Android SDK Platform**: `Android SDK Platform 13`
- **Android API Level**: `33`
- **Android NDK**: `26.2.11394342`
- **Android Build Tools**: `34.0.0`
