INSTALL=install
INSTALL_PROGRAM=$(INSTALL)
INSTALL_DATA=$(INSTALL) -m 644

bin_dir=$(HOME)/.local/bin
data_dir=$(HOME)/.local/share
config_dir=$(HOME)/.config
name=com.github.ankjevel.tick-tock

.PHONY : clean install uninstall

target/release/tick-tock : src
	cargo build --release

install : target/release/tick-tock
	# Install binary
	$(INSTALL_PROGRAM) target/release/tick-tock $(bin_dir)/$(name)

	# Install config
	$(INSTALL_DATA) config.json $(config_dir)/$(name).json

	# Install icon(s)
	# $(INSTALL_DATA) data/$(name).svg $(data_dir)/icons/hicolor/scalable/apps/$(name).svg
	# $(INSTALL_DATA) data/$(name).64.png $(data_dir)/icons/hicolor/64x64/apps/$(name).png
	# $(INSTALL_DATA) data/$(name).128.png $(data_dir)/icons/hicolor/128x128/apps/$(name).png

	# Force icon cache refresh
	# touch $(data_dir)/icons/hicolor

	# Install desktop file
	$(INSTALL_DATA) data/$(name).desktop $(data_dir)/applications/$(name).desktop

uninstall :
	# Remove the desktop file
	rm -f $(data_dir)/applications/$(name).desktop

	# Remove the icon(s)
	# rm -f $(data_dir)/icons/hicolor/scalable/apps/$(name).svg
	# rm -f $(data_dir)/icons/hicolor/64x64/apps/$(name).png
	# rm -f $(data_dir)/icons/hicolor/128x128/apps/$(name).png

	# Remove the config
	rm -f $(config_dir)/$(name).json

	# Remove the binary
	rm -f $(bin_dir)/bin/$(name)

clean :
	cargo clean

