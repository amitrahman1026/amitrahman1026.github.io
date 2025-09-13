---
title: "Modern CMake: A Mental Model for Large Codebases (part I)"
description: "An intuition for CMake internals with many practical examples"
date: "2025-09-13"
tags: ["cmake", "cpp", "build systems"]
draft: false
---

## Why another CMake tutorial?

In this article, I will attempt to create a better intuition for how CMake works, so the next time you modify your CMakeLists.txt, you can potentially leave it in a better state than you found it in. 

**This article is not introductory. It is aimed at developers who may be working on established codebases with mixes CMake practices. To learn more about CMake, you may visit any of the links in this aritcle or the [official documentations](https://cmake.org/cmake/help/latest/guide/tutorial/index.html).**

CMake is essentially a domain-specific language for a general purpose build system. There are great "Modern CMake" tutorials floating around like [An introduction to Modern CMake](https://cliutils.gitlab.io/modern-cmake/README.html), and presentations like [More Modern CMake - Deniz Bahadir - Meeting C++ 2018](https://www.youtube.com/watch?v=y7ndUhdQuU8). However one area some of these resources are lacking in -  in creating an understanding of "what" your CMake is doing. Due to abstraction, there is a disconnect between what compile commands your make file CMake would generate and what you write in your CMakeLists.txt.

When you are trying to debug what wrong, it really helps if you are able to operate from first principles.

From taking a look at various cookbooks, to comparing how CMake is done in various open source projects - popular library collections like [Boost](https://github.com/boostorg/boost) to applications like [ScyllaDB](https://github.com/scylladb/scylladb), to templates & generators in [Awsome CMake](https://github.com/onqtam/awesome-cmake?tab=readme-ov-file#examples--templates), I've tried to collate some good practices that we should be concious of when writing.

At the end of this, by understanding the underlying object model, property propagation system, and dependency resolution algorithm, you should be able to:

- Predict what compiler flags will be generated
- Debug dependency issues
- Design scalable build systems
- Leverage advanced features confidently
- Have a mental checklist of Dos & Don'ts

n/b: Article below uses various LLM generated examples for illustration. Ideas, explanations & presentation are my own, in reference to other aritcles & documentations.

## Core Philosophy: Encapsulation & Modularisation

CMake's build system emphasises encapsulation & modularisation - afterall that's exactly why you see the recursive CMakeLists.txt files. It's definitely daunting to look at a large codebase where a lot of the CMake sometimes doesn't make sense by "Modern" practices, like global macro injects. But the good thing is due to the ability to create build units, you can incrementally tackle improvements.

### Targets as First-Class Objects

Modern CMake fundamentally revolves around **targets** - abstract objects that represent something to be built. Think of CMake as maintaining a graph database where **nodes are targets** and **edges represent dependencies and property inheritance**. 

### The Target Object Model

Every target in CMake can be conceptually represented as:

```
Target := {
    name: String,
    type: TargetType,
    sources: List<SourceFile>,
    properties: PropertyBag,
    dependencies: List<Target>,
    usage_requirements: PropertyBag
}

TargetType := EXECUTABLE | STATIC_LIBRARY | SHARED_LIBRARY | INTERFACE | OBJECT | IMPORTED
```

The key insight is that targets carry two distinct sets of properties:
- **Build properties**: What this target needs to be built correctly
- **Usage requirements**: What other targets need when they depend on this target

### Property Propagation Model

Just like object oriented programming models, CMake implements an inheritance system with three visibility levels for their properties:

```
PropertyVisibility := PRIVATE | INTERFACE | PUBLIC

where:
- PRIVATE: Target → Target (build requirements only)
- INTERFACE: Target → Dependents (usage requirements only)  
- PUBLIC: Target → Target ∪ Dependents (both)
```

This creates a directed acyclic graph (DAG) where properties flow from dependencies to dependents:

```
A --depends-on--> B --depends-on--> C

Property flow: C.interface → B.build ∪ B.interface → A.build
```

CMake automatically deduplicates edges in this dependency graph. Even in large code bases, explicitness if preferred. **Include what you use.** Don't blindly rely on transitivity. 

## Target Types

Working on CMake managed projects, you will typically work with a handful of target types. Each target would translate directly to what you would see if you were trying to write a command to compile the said target. 

### Executable Targets
```
ExecutableTarget := {
    type: EXECUTABLE,
    output: BinaryFile,
    link_libraries: List<Target>,
    compile_definitions: PropertyBag,
    include_directories: PropertyBag
}
```

**Compiler Translation**: Becomes the final link command with all transitive dependencies resolved:

```bash
g++ main.o lib1.a lib2.so -I/include1 -I/include2 -DFLAG1 -o executable
```

### Library Targets

#### Static Libraries
```
StaticLibraryTarget := {
    type: STATIC_LIBRARY,
    output: ArchiveFile,
    sources: List<SourceFile>,
    usage_requirements: {
        interface_include_directories: List<Path>,
        interface_compile_definitions: List<String>,
        interface_link_libraries: List<Target>
    }
}
```

**Compiler Translation**: 

Typical flow if you were to build a `libname.a` static library by hand:

1. Compile phase: `g++ -c source.cpp -I/private/includes -o source.o`
2. Archive phase: `ar rcs libname.a source1.o source2.o`

#### Shared Libraries
```
SharedLibraryTarget := {
    type: SHARED_LIBRARY,
    output: DynamicLibraryFile,
    sources: List<SourceFile>,
    link_libraries: List<Target>,
    usage_requirements: PropertyBag
}
```

**Key Difference**: Shared libraries resolve their dependencies at build time, so transitive dependencies become part of the link line.

#### Interface Libraries
```
InterfaceTarget := {
    type: INTERFACE,
    sources: ∅,
    output: ∅,
    usage_requirements: PropertyBag
}
```

Interface libraries are purely abstractions. This isn't something concrete that will be compiled. They exist only to carry usage requirements. Think of them as "property bundles" that can be attached to real targets, grouping together various dependencies. Some common use cases include:

- Header-only libraries
- Compiler flag collections
- Third-party library wrappers
- Build configuration presets

## The CMake Processing Pipeline

The next part you will need to understand is what happens when you configure cmake, then build.

### Phase 1: Configuration - `cmake -B build_dir -S src_dir`
```
Configuration := {
    parse_cmakelists() →
    create_target_graph() →
    resolve_generator_expressions() →
    validate_dependencies() →
    output: BuildSystemFiles
}
```

During configuration, CMake builds an in-memory representation of all targets and their relationships. No compilation occurs.

### Phase 2: Generation

This is pretty tightly coupled with phase 1, where CMake translates the target graph into build system files (Makefiles, Ninja files, VS projects, etc.). 

This is also where the abstract target model gets converted into concrete build commands.

### Phase 3: Build (make/ninja)
This is where you will typically see in your `build_dir` the generated build files, arranges in a file directory structure that mirrors your `src_dir`. Here the build system recursively reads the generated files and executes the actual compilation/linking.

## Dependency Resolution Algorithm

One part I'd alluded to before when advising to include what you use, is that CMake does dependency resolution which can roughly be modeled as a graph search where:

- Transitive dependencies are resolved and flattened.
- Properties are deduplicated (same -I/path won't appear twice)
- Order is preserved where it matters (link order)
- Circular dependencies are detected and rejected

```
function resolve_dependencies(target: Target) → CompileFlags:
    visited = ∅
    result = ∅
    
    function dfs(t: Target, visibility: Visibility):
        if t ∈ visited: return
        visited.add(t)
        
        if visibility ≠ PRIVATE:
            result.merge(t.usage_requirements)
        
        for dep in t.dependencies:
            new_visibility = combine_visibility(visibility, dep.visibility)
            dfs(dep.target, new_visibility)
    
    dfs(target, PRIVATE)
    return deduplicate(result)
```

## Property Types and Their Translations

The following section is going to cover what 90% of your CMake code is doing and how it translates to a target's compilation command. 

Before we go any further, `compile_commands.json` is an important generated file that you should be familiar with. It's effectively the final build dependency graph. Not only is it useful to help your LSP like `clangd` provide definitions in your editor, it is an excellent way to debug build issues, providing the exact compiler command being run on your source along with real directory paths.

You may generate it with `cmake -B build_dir -S src_dir -DCMAKE_EXPORT_COMPILE_COMMANDS=ON`. 

### Include Directories

```
target_include_directories(mylib PUBLIC include/)
```

**Internal Representation**:
```
mylib.interface_include_directories += ["include/"]
```

**Compiler Translation**: `-I/path/to/include/`

**In compile_commands.json**:

```json
{
    "arguments": [
        "g++", "-I/path/to/include/", "source.cpp"
    ]
}
```

### Compile Definitions
```
target_compile_definitions(mylib PRIVATE DEBUG=1)
```

**Internal Representation**:
```
mylib.compile_definitions += ["DEBUG=1"]
```

**Compiler Translation**: `-DDEBUG=1`

### Link Libraries
```
target_link_libraries(app PRIVATE mylib)
```

**Internal Representation**:
```
app.dependencies += [{target: mylib, visibility: PRIVATE}]
```

**Compiler Translation**: The actual library file path or `-lmylib`

## Generator Expressions: Conditional Logic

Generator expressions evaluate during the generation phase, after CMake parses all your CMakeLists.txt files but before it writes build system files. They're essentially lazy evaluation and CMake applies it per configuration run.

```
GeneratorExpression := $<condition:true_value:false_value>

Examples:
$<CONFIG:Debug>           → True if building Debug configuration
$<TARGET_PROPERTY:t,prop> → Value of property 'prop' on target 't'
$<BOOL:value>            → Convert to boolean
```

The syntax is terse, but instead of writing separate CMake branches for Debug vs Release, you embed the conditional logic directly:

```cmake
target_compile_definitions(mylib PRIVATE
    $<$<CONFIG:Debug>:DEBUG_MODE>
    $<$<CONFIG:Release>:NDEBUG>
    VERSION_STRING="$<TARGET_PROPERTY:mylib,VERSION>"
)
```

**Compiler Translation**: For Debug builds, this becomes the injected macro `-DDEBUG_MODE -DVERSION_STRING="1.2.3"` and for Release builds: `-DNDEBUG -DVERSION_STRING="1.2.3"`.

The nested `$<$<CONFIG:Debug>:DEBUG_MODE>` pattern is common - the outer expression evaluates the inner condition and returns the value if true, empty string if false.

---

With that, I'd like to wrap up the first part of the CMake series. Stay tuned for the second part of this series where I will go over some specific examples of things to look out for, to have the ability to positively impact CMake whereever you go.

