-- set minimum xmake version
set_xmakever("3.0.0")

add_rules("mode.debug", "mode.release")

-- 1. ОБЪЯВЛЯЕМ ОПЦИИ (ЗНАЧЕНИЯ ПРИДУТ ИЗ RUST BUILD.RS)
option("skyrim_se")
    set_default(true)
    set_showmenu(true)
option_end()

option("skyrim_ae")
    set_default(true)
    set_showmenu(true)
option_end()

option("skyrim_vr")
    set_default(false)
    set_showmenu(true)
option_end()

option("skse_xbyak")
    set_default(true)
    set_showmenu(true)
option_end()

-- ФИКС: Объявляем дефайны глобально ДО всего остального,
-- проверяя переданные из Rust опции
if get_config("skyrim_se") then add_defines("ENABLE_SKYRIM_SE=1") end
if get_config("skyrim_ae") then add_defines("ENABLE_SKYRIM_AE=1") end
if get_config("skyrim_vr") then add_defines("ENABLE_SKYRIM_VR=1") end
if get_config("skse_xbyak") then add_defines("SKSE_SUPPORT_XBYAK=1") end
add_defines("ENABLE_COMMONLIBSSE_TESTING=1")

-- 2. ОПРЕДЕЛЯЕМ КАСТОМНЫЙ ПАКЕТ
package("commonlibsse-ng")
           -- https://github.com/alandtse/CommonLibVR/archive/refs/tags/v4.21.1.zip
    add_urls("https://github.com/alandtse/CommonLibVR.git")
    add_versions("v4.21.1", "6ba6e159805f7e9ebbd7b232e7ef5c1b86db3d23")

    add_deps("directxmath 2024.02", "directxtk 24.2.0")
    add_deps("spdlog v1.16.0", { configs = { header_only = false, wchar = true, std_format = true } })
    add_deps("xbyak v7.06")

    on_install(function (package)
        local configs = {}
        -- Передаем опции дальше в сборку самого CommonLib-NG
        table.insert(configs, "--skyrim_se="  .. (get_config("skyrim_se")  and "y" or "n"))
        table.insert(configs, "--skyrim_ae="  .. (get_config("skyrim_ae")  and "y" or "n"))
        table.insert(configs, "--skyrim_vr="  .. (get_config("skyrim_vr")  and "y" or "n"))
        table.insert(configs, "--skse_xbyak=" .. (get_config("skse_xbyak") and "y" or "n"))

        import("package.tools.xmake").install(package, configs)
    end)
package_end()

-- 3. ТРЕБУЕМ НАШ ПАКЕТ
add_requires("commonlibsse-ng v4.21.1")
add_requires("minhook")

-- 4. НАША ЦЕЛЬ (C++ Мост для Rust)
target("commonlib_bridge")
    set_kind("static")
    set_languages("c++23")

    -- ФИКС MSVC: Включаем новый препроцессор и строгое соответствие C++
    if is_plat("windows") then
        add_cxxflags("cl::/Zc:preprocessor", "cl::/permissive-", "cl::/EHsc")
    end

    -- Отключаем предупреждения
    add_cxxflags("/wd4005", "/wd4061", "/wd4068", "/wd4200", "/wd4201")

    add_includedirs("include")
    set_pcxxheader("include/PCH.h")

    -- Подключаем пакет
    add_packages("commonlibsse-ng", "minhook")

    -- Исходники моста
    add_headerfiles("include/**.h", "include/**.hpp")
    add_files("src/**.cpp")
