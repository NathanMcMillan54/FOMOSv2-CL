define inner-virtual-package

# Ensure the virtual package has an implementation defined.
ifeq ($$(BR2_PACKAGE_HAS_$(2)),y)
ifeq ($$(call qstrip,$$(BR2_PACKAGE_PROVIDES_$(2))),)
$$(error No implementation selected for virtual package $(1). Configuration error)
endif
endif

# A virtual package does not have any source associated
$(2)_SOURCE =

# Fake a version string, so it looks nicer in the build log
$(2)_VERSION = virtual

# This must be repeated from inner-generic-package, otherwise we get an empty
# _DEPENDENCIES
ifeq ($(4),host)
$(2)_DEPENDENCIES ?= $$(filter-out host-toolchain $(1),\
	$$(patsubst host-host-%,host-%,$$(addprefix host-,$$($(3)_DEPENDENCIES))))
endif

# Add dependency against the provider
$(2)_DEPENDENCIES += $$(call qstrip,$$(BR2_PACKAGE_PROVIDES_$(2)))

# Call the generic package infrastructure to generate the necessary
# make targets
$(call inner-generic-package,$(1),$(2),$(3),$(4))

endef

################################################################################
# virtual-package -- the target generator macro for virtual packages
################################################################################

virtual-package = $(call inner-virtual-package,$(pkgname),$(call UPPERCASE,$(pkgname)),$(call UPPERCASE,$(pkgname)),target)
host-virtual-package = $(call inner-virtual-package,host-$(pkgname),$(call UPPERCASE,host-$(pkgname)),$(call UPPERCASE,$(pkgname)),host)
