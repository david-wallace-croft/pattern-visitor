use super::element::Element;

pub trait ClusterElement: Element {
  fn cluster(
    &mut self,
    cluster_elements: &[Box<dyn ClusterElement>],
  );

  fn overlaps(
    &self,
    other: &dyn ClusterElement,
  ) -> bool;
}

impl<T: Element> ClusterElement for T {
  fn cluster(
    &mut self,
    cluster_elements: &[Box<dyn ClusterElement>],
  ) {
    let id: usize = self.get_id();

    for cluster_element in cluster_elements {
      if cluster_element.get_id() == id {
        continue;
      }

      if self.overlaps(&**cluster_element) {
        todo!()
      }

      todo!()
    }
  }

  fn overlaps(
    &self,
    _other: &dyn ClusterElement,
  ) -> bool {
    todo!()
  }
}
