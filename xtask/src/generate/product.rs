pub(crate) const PRODUCTS: [Product; 3] = [
    Product { base: "Automation", module: "automation", path_prefix: "" },
    Product { base: "Confluence", module: "v1", path_prefix: "" },
    Product { base: "Confluence", module: "v2", path_prefix: "/wiki/api/v2" },
];

#[derive(Clone, Copy, Debug)]
pub(crate) struct Product {
    pub base: &'static str,
    pub module: &'static str,
    pub path_prefix: &'static str,
}
