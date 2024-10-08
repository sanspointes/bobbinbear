import { EmbDocument } from '@/store/documentStore';
import { NewDocumentForm } from './NewDocumentForm';
import { Modal, ModalProps } from '@/components/generics/Modal';

type NewDocumentModalProps = {
    isCancellable: boolean;
    onCreate: (doc: EmbDocument) => void;
    onClose: ModalProps['onClose'];
};
export function NewDocumentModal(props: NewDocumentModalProps) {
    return (
        <Modal
            open={true}
            onClose={props.onClose}
            disableClose={!props.isCancellable}
            title="Add a new design"
            class="max-w-[90vw] w-[400px] sm:w-[600px]"
        >
            <NewDocumentForm
                onCreate={props.onCreate}
                onCancel={() => props.onClose && props.onClose()}
                disableCancel={!props.isCancellable}
            />
        </Modal>
    );
}
