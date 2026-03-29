use lilv::plugin::Plugin;

pub struct Lv2World {
    world: lilv::World,
}

impl Lv2World {
    #[must_use]
    pub fn load() -> Self {
        let world = lilv::World::new();
        world.load_all();
        Self { world }
    }

    #[must_use]
    pub fn world(&self) -> &lilv::World {
        &self.world
    }

    #[must_use]
    pub fn find_plugin(&self, plugin_uri: &str) -> Option<Plugin> {
        let plugin_uri_node = self.world.new_uri(plugin_uri);
        self.world.plugins().plugin(&plugin_uri_node)
    }

    pub fn plugins(&self) -> Vec<Plugin> {
        self.world.plugins().iter().collect()
    }
}
